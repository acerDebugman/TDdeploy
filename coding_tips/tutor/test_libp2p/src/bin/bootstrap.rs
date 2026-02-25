use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use clap::Parser;
use futures::StreamExt;
use libp2p::{
    autonat,
    core::{multiaddr::Protocol, Multiaddr},
    identify, identity, kad, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let opt = Opt::parse();

    // Create a static known PeerId based on given secret
    let local_key: identity::Keypair = generate_ed25519(opt.secret_key_seed);
    let local_peer_id = local_key.public().to_peer_id();
    
    println!("Bootstrap Node PeerId: {local_peer_id}");

    // Create relay behaviour config
    let relay_config = relay::Config::default();
    
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic() // Also support QUIC
        .with_behaviour(|key| {
            // Create relay behaviour - bootstrap node acts as relay server
            let relay_behaviour = relay::Behaviour::new(local_peer_id, relay_config);
            
            // Create autonat behaviour - for detecting NAT status
            // Bootstrap node runs as autonat server (allows other nodes to probe)
            let autonat_config = autonat::Config {
                only_global_ips: false,
                throttle_clients_global_max: 30,
                throttle_clients_peer_max: 3,
                throttle_clients_period: Duration::from_secs(60),
                ..Default::default()
            };
            let autonat_behaviour = autonat::Behaviour::new(local_peer_id, autonat_config);
            
            Behaviour {
                relay: relay_behaviour,
                kademlia: kad::Behaviour::new(
                    local_peer_id,
                    kad::store::MemoryStore::new(local_peer_id),
                ),
                ping: ping::Behaviour::new(ping::Config::new()),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/p2p-bootstrap/0.1.0".to_string(),
                    key.public(),
                )),
                autonat: autonat_behaviour,
            }
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen on TCP
    let listen_addr_tcp = Multiaddr::empty()
        .with(match opt.use_ipv6 {
            Some(true) => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Tcp(opt.port));
    swarm.listen_on(listen_addr_tcp)?;
    
    // Listen on QUIC
    let listen_addr_quic = Multiaddr::empty()
        .with(match opt.use_ipv6 {
            Some(true) => Protocol::from(Ipv6Addr::UNSPECIFIED),
            _ => Protocol::from(Ipv4Addr::UNSPECIFIED),
        })
        .with(Protocol::Udp(opt.port))
        .with(Protocol::QuicV1);
    swarm.listen_on(listen_addr_quic)?;

    println!("\n=== Bootstrap Node Started ===");
    println!("PeerId: {local_peer_id}");
    println!("TCP Port: {}", opt.port);
    println!("QUIC Port: {}", opt.port);
    println!("\nFeatures enabled:");
    println!("  - Kademlia DHT (node discovery)");
    println!("  - Relay Server (circuit relay)");
    println!("  - AutoNAT Server (NAT detection)");
    println!("  - Identify Protocol");
    println!("  - Ping Protocol");
    println!("\nConnection strings:");
    println!("  TCP:  /ip4/<ip>/tcp/{}/p2p/{}", opt.port, local_peer_id);
    println!("  QUIC: /ip4/<ip>/udp/{}/quic-v1/p2p/{}", opt.port, local_peer_id);
    println!("==============================\n");

    loop {
        match swarm.next().await.expect("Infinite Stream.") {
            SwarmEvent::Behaviour(event) => match event {
                BehaviourEvent::Identify(identify::Event::Received {
                    peer_id,
                    info,
                    ..
                }) => {
                    println!("[Identify] Peer {} identified, protocol version: {:?}, agent: {:?}", 
                        peer_id, 
                        info.protocol_version,
                        info.agent_version
                    );
                    println!("  Listen addrs: {:?}", info.listen_addrs);
                    println!("  Observed addr: {:?}", info.observed_addr);
                    
                    for addr in &info.listen_addrs {
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                    }
                    // Also add observed address
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, info.observed_addr.clone());
                }
                BehaviourEvent::Identify(identify::Event::Sent { peer_id, .. }) => {
                    println!("[Identify] Sent identify info to {}", peer_id);
                }
                BehaviourEvent::Identify(other) => {
                    tracing::debug!("[Identify] Other event: {:?}", other);
                }
                BehaviourEvent::Kademlia(kad::Event::RoutingUpdated { peer, is_new_peer, addresses, bucket_range, .. }) => {
                    println!("[Kademlia] Routing updated: peer={}", peer);
                    if is_new_peer {
                        println!("  -> New peer added to routing table");
                    }
                    println!("  Addresses: {:?}", addresses);
                    println!("  Bucket range: {:?}", bucket_range);
                }
                BehaviourEvent::Kademlia(kad::Event::UnroutablePeer { peer, .. }) => {
                    println!("[Kademlia] Peer {} is unroutable", peer);
                }
                BehaviourEvent::Kademlia(kad::Event::RoutablePeer { peer, address, .. }) => {
                    println!("[Kademlia] Peer {} is routable via {}", peer, address);
                }
                BehaviourEvent::Kademlia(other) => {
                    tracing::debug!("[Kademlia] Other event: {:?}", other);
                }
                BehaviourEvent::Ping(event) => {
                    match event {
                        ping::Event {
                            peer,
                            result: Ok(rtt),
                            connection,
                        } => {
                            tracing::debug!("[Ping] {} rtt: {:?} (conn: {:?})", peer, rtt, connection);
                        }
                        ping::Event {
                            peer,
                            result: Err(e),
                            connection,
                        } => {
                            println!("[Ping] {} failed: {:?} (conn: {:?})", peer, e, connection);
                        }
                    }
                }
                BehaviourEvent::Relay(event) => {
                    match event {
                        relay::Event::ReservationReqAccepted { src_peer_id, renewed, .. } => {
                            if renewed {
                                println!("[Relay] Reservation renewed for {}", src_peer_id);
                            } else {
                                println!("[Relay] New reservation accepted from {}", src_peer_id);
                            }
                        }
                        relay::Event::ReservationReqDenied { src_peer_id, .. } => {
                            println!("[Relay] Reservation denied for {}", src_peer_id);
                        }
                        relay::Event::ReservationTimedOut { src_peer_id, .. } => {
                            println!("[Relay] Reservation timed out for {}", src_peer_id);
                        }
                        relay::Event::CircuitReqAccepted { src_peer_id, dst_peer_id, .. } => {
                            println!("[Relay] Circuit established: {} -> {}", src_peer_id, dst_peer_id);
                        }
                        relay::Event::CircuitReqDenied { src_peer_id, dst_peer_id, .. } => {
                            println!("[Relay] Circuit denied: {} -> {}", src_peer_id, dst_peer_id);
                        }
                        relay::Event::CircuitClosed { src_peer_id, dst_peer_id, error, .. } => {
                            if let Some(err) = error {
                                println!("[Relay] Circuit closed with error: {} -> {}, error: {:?}", 
                                    src_peer_id, dst_peer_id, err);
                            } else {
                                tracing::debug!("[Relay] Circuit closed: {} -> {}", src_peer_id, dst_peer_id);
                            }
                        }
                        _ => {
                            tracing::debug!("[Relay] Other event: {:?}", event);
                        }
                    }
                }
                BehaviourEvent::Autonat(event) => {
                    match event {
                        autonat::Event::InboundProbe(event) => {
                            match event {
                                autonat::InboundProbeEvent::Request { peer, .. } => {
                                    println!("[AutoNAT] Inbound probe request from {}", peer);
                                }
                                autonat::InboundProbeEvent::Response { peer, address, .. } => {
                                    println!("[AutoNAT] Inbound probe response to {}, address: {}", peer, address);
                                }
                                autonat::InboundProbeEvent::Error { peer, error, .. } => {
                                    println!("[AutoNAT] Inbound probe error from {}: {:?}", peer, error);
                                }
                            }
                        }
                        autonat::Event::OutboundProbe(event) => {
                            tracing::debug!("[AutoNAT] Outbound probe: {:?}", event);
                        }
                        autonat::Event::StatusChanged { old, new } => {
                            println!("[AutoNAT] Status changed: {:?} -> {:?}", old, new);
                        }
                    }
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("[Swarm] Listening on {}", address);
            }
            SwarmEvent::NewExternalAddrOfPeer { peer_id, address } => {
                println!("[Swarm] New external address for {}: {}", peer_id, address);
            }
            SwarmEvent::IncomingConnection { send_back_addr, local_addr, .. } => {
                println!("[Swarm] Incoming connection from {} (local: {})", send_back_addr, local_addr);
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, num_established, .. } => {
                println!("[Swarm] Connected to {} via {:?} (total: {})", 
                    peer_id, endpoint, num_established);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, num_established, .. } => {
                let cause_str = cause.map(|c| format!("{:?}", c)).unwrap_or_else(|| "normal".to_string());
                println!("[Swarm] Disconnected from {}: {} (remaining: {})", 
                    peer_id, cause_str, num_established);
            }
            SwarmEvent::ExternalAddrConfirmed { address } => {
                println!("[Swarm] External address confirmed: {}", address);
            }
            SwarmEvent::ExternalAddrExpired { address } => {
                println!("[Swarm] External address expired: {}", address);
            }
            SwarmEvent::ListenerClosed { addresses, reason, .. } => {
                println!("[Swarm] Listener closed: {:?}, reason: {:?}", addresses, reason);
            }
            SwarmEvent::ListenerError { error, .. } => {
                eprintln!("[Swarm] Listener error: {:?}", error);
            }
            other => {
                tracing::trace!("[Swarm] Other event: {:?}", other);
            }
        }
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    relay: relay::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    autonat: autonat::Behaviour,
}

fn generate_ed25519(secret_key_seed: u8) -> identity::Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;
    identity::Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}

#[derive(Debug, Parser)]
#[command(name = "bootstrap")]
struct Opt {
    /// Determine if the relay listen on ipv6 or ipv4 loopback address. the default is ipv4
    #[arg(long)]
    use_ipv6: Option<bool>,

    /// Fixed value to generate deterministic peer id
    #[arg(long, default_value = "0")]
    secret_key_seed: u8,

    /// The port used to listen on all interfaces
    #[arg(long, default_value = "9090")]
    port: u16,
}
