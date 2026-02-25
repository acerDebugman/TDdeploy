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
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing with default log level "info" if RUST_LOG is not set
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();

    let opt = Opt::parse();

    // Create a static known PeerId based on given secret
    let local_key: identity::Keypair = generate_ed25519(opt.secret_key_seed);
    let local_peer_id = local_key.public().to_peer_id();
    
    info!("Bootstrap Node PeerId: {local_peer_id}");

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

    info!("=== Bootstrap Node Started ===");
    info!("PeerId: {local_peer_id}");
    info!("TCP Port: {}", opt.port);
    info!("QUIC Port: {}", opt.port);
    info!("Features: Kademlia DHT, Relay Server, AutoNAT Server, Identify, Ping");
    info!("TCP: /ip4/<ip>/tcp/{}/p2p/{}", opt.port, local_peer_id);
    info!("QUIC: /ip4/<ip>/udp/{}/quic-v1/p2p/{}", opt.port, local_peer_id);

    loop {
        match swarm.next().await.expect("Infinite Stream.") {
            SwarmEvent::Behaviour(event) => match event {
                BehaviourEvent::Identify(identify::Event::Received {
                    peer_id,
                    info,
                    ..
                }) => {
                    info!("[Identify] Peer {} identified, protocol version: {:?}, agent: {:?}", 
                        peer_id, 
                        info.protocol_version,
                        info.agent_version
                    );
                    info!("[Identify] Listen addrs: {:?}", info.listen_addrs);
                    info!("[Identify] Observed addr: {:?}", info.observed_addr);
                    
                    for addr in &info.listen_addrs {
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                    }
                    // Also add observed address
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, info.observed_addr.clone());
                }
                BehaviourEvent::Identify(identify::Event::Sent { peer_id, .. }) => {
                    info!("[Identify] Sent identify info to {}", peer_id);
                }
                BehaviourEvent::Identify(other) => {
                    tracing::debug!("[Identify] Other event: {:?}", other);
                }
                BehaviourEvent::Kademlia(kad::Event::RoutingUpdated { peer, is_new_peer, addresses, .. }) => {
                    if is_new_peer {
                        info!("[Kademlia] New peer added: {}, addresses: {:?}", peer, addresses);
                    } else {
                        info!("[Kademlia] Routing updated: peer={}", peer);
                    }
                }
                BehaviourEvent::Kademlia(kad::Event::UnroutablePeer { peer, .. }) => {
                    info!("[Kademlia] Peer {} is unroutable", peer);
                }
                BehaviourEvent::Kademlia(kad::Event::RoutablePeer { peer, address, .. }) => {
                    info!("[Kademlia] Peer {} is routable via {}", peer, address);
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
                            warn!("[Ping] {} failed: {:?} (conn: {:?})", peer, e, connection);
                        }
                    }
                }
                BehaviourEvent::Relay(event) => {
                    match event {
                        relay::Event::ReservationReqAccepted { src_peer_id, renewed, .. } => {
                            if renewed {
                                info!("[Relay] Reservation renewed for {}", src_peer_id);
                            } else {
                                info!("[Relay] New reservation accepted from {}", src_peer_id);
                            }
                        }
                        relay::Event::ReservationReqDenied { src_peer_id, .. } => {
                            warn!("[Relay] Reservation denied for {}", src_peer_id);
                        }
                        relay::Event::ReservationTimedOut { src_peer_id, .. } => {
                            info!("[Relay] Reservation timed out for {}", src_peer_id);
                        }
                        relay::Event::CircuitReqAccepted { src_peer_id, dst_peer_id, .. } => {
                            info!("[Relay] Circuit established: {} -> {}", src_peer_id, dst_peer_id);
                        }
                        relay::Event::CircuitReqDenied { src_peer_id, dst_peer_id, .. } => {
                            warn!("[Relay] Circuit denied: {} -> {}", src_peer_id, dst_peer_id);
                        }
                        relay::Event::CircuitClosed { src_peer_id, dst_peer_id, error, .. } => {
                            if let Some(err) = error {
                                warn!("[Relay] Circuit closed with error: {} -> {}, error: {:?}", 
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
                                    info!("[AutoNAT] Inbound probe request from {}", peer);
                                }
                                autonat::InboundProbeEvent::Response { peer, address, .. } => {
                                    info!("[AutoNAT] Inbound probe response to {}, address: {}", peer, address);
                                }
                                autonat::InboundProbeEvent::Error { peer, error, .. } => {
                                    warn!("[AutoNAT] Inbound probe error from {}: {:?}", peer, error);
                                }
                            }
                        }
                        autonat::Event::OutboundProbe(event) => {
                            tracing::debug!("[AutoNAT] Outbound probe: {:?}", event);
                        }
                        autonat::Event::StatusChanged { old, new } => {
                            info!("[AutoNAT] Status changed: {:?} -> {:?}", old, new);
                        }
                    }
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("[Swarm] Listening on {}", address);
            }
            SwarmEvent::NewExternalAddrOfPeer { peer_id, address } => {
                info!("[Swarm] New external address for {}: {}", peer_id, address);
            }
            SwarmEvent::IncomingConnection { send_back_addr, local_addr, .. } => {
                info!("[Swarm] Incoming connection from {} (local: {})", send_back_addr, local_addr);
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, num_established, .. } => {
                info!("[Swarm] Connected to {} via {:?} (total: {})", 
                    peer_id, endpoint, num_established);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, num_established, .. } => {
                let cause_str = cause.map(|c| format!("{:?}", c)).unwrap_or_else(|| "normal".to_string());
                info!("[Swarm] Disconnected from {}: {} (remaining: {})", 
                    peer_id, cause_str, num_established);
            }
            SwarmEvent::ExternalAddrConfirmed { address } => {
                info!("[Swarm] External address confirmed: {}", address);
            }
            SwarmEvent::ExternalAddrExpired { address } => {
                info!("[Swarm] External address expired: {}", address);
            }
            SwarmEvent::ListenerClosed { addresses, reason, .. } => {
                warn!("[Swarm] Listener closed: {:?}, reason: {:?}", addresses, reason);
            }
            SwarmEvent::ListenerError { error, .. } => {
                warn!("[Swarm] Listener error: {:?}", error);
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
