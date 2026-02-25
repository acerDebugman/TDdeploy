use std::error::Error;
use std::time::Duration;

use futures::StreamExt;
use libp2p::{
    core::multiaddr::Protocol,
    identify, identity, kad, mdns, noise, ping, request_response::{self, cbor, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

// P2P request/response types (must match node_b)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2PRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2PResponse {
    pub success: bool,
    pub data: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing with default log level if RUST_LOG is not set
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let p2p_port = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10002u16);
    let bootstrap_addr = args.get(2).cloned();
    // Support direct connection to node_b
    let target_node_b_addr = args.get(3).cloned();

    println!("Starting Node A - P2P port: {}", p2p_port);
    info!("Starting Node A - P2P port: {}", p2p_port);

    // Create identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();
    println!("Local PeerId: {}", local_peer_id);
    info!("Local PeerId: {}", local_peer_id);

    // Setup request-response protocol - use string protocol name
    let protocols = std::iter::once((StreamProtocol::new("/p2p-api/1.0.0"), ProtocolSupport::Full));
    let cfg = request_response::Config::default().with_request_timeout(Duration::from_secs(30));
    let rr_behaviour = cbor::Behaviour::<P2PRequest, P2PResponse>::new(protocols, cfg);

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| Behaviour {
            kademlia: kad::Behaviour::new(
                local_peer_id,
                kad::store::MemoryStore::new(local_peer_id),
            ),
            identify: identify::Behaviour::new(identify::Config::new(
                "/p2p-node/0.1.0".to_string(),
                key.public(),
            )),
            request_response: rr_behaviour,
            ping: ping::Behaviour::new(ping::Config::new()),
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)
                .expect("Failed to create mDNS behaviour"),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen on P2P port
    let listen_addr: Multiaddr = Multiaddr::empty()
        .with(Protocol::Ip4([0, 0, 0, 0].into()))
        .with(Protocol::Tcp(p2p_port));
    swarm.listen_on(listen_addr)?;

    // Connect to bootstrap node
    let mut bootstrap_peer_id: Option<PeerId> = None;
    let mut bootstrap_connected = false;
    
    if let Some(addr) = bootstrap_addr.clone() {
        let bootstrap_multiaddr: Multiaddr = addr.parse()?;
        println!("Connecting to bootstrap node: {}", bootstrap_multiaddr);
        info!("Connecting to bootstrap node: {}", bootstrap_multiaddr);
        swarm.dial(bootstrap_multiaddr)?;
    } else {
        println!("Warning: No bootstrap address provided, will rely on mDNS discovery");
    }

    // Direct dial to node_b if address provided
    if let Some(addr_str) = target_node_b_addr {
        println!("Will dial node B directly at: {}", addr_str);
        match addr_str.parse::<Multiaddr>() {
            Ok(addr) => {
                if let Err(e) = swarm.dial(addr.clone()) {
                    warn!("Failed to dial node B: {:?}", e);
                } else {
                    println!("Dialing node B at {}", addr);
                }
            }
            Err(e) => {
                warn!("Invalid node B address '{}': {:?}", addr_str, e);
            }
        }
    }

    // Store discovered peers
    let mut discovered_peers: Vec<PeerId> = Vec::new();
    let mut connected_to_b = false;
    let mut identified_node_b: Option<PeerId> = None;  // Track identified Node B

    // Main P2P loop
    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                        for (peer_id, addr) in peers {
                            println!("[mDNS] Discovered peer {} at {}", peer_id, addr);
                            info!("[mDNS] Discovered peer {} at {}", peer_id, addr);
                            
                            if peer_id != local_peer_id && !connected_to_b {
                                // Add to Kademlia
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                                
                                // Try to dial
                                let dial_addr = addr.with(Protocol::P2p(peer_id));
                                println!("[mDNS] Dialing discovered peer at {}", dial_addr);
                                if let Err(e) = swarm.dial(dial_addr) {
                                    warn!("[mDNS] Failed to dial peer: {:?}", e);
                                }
                            }
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                        for (peer_id, addr) in peers {
                            println!("[mDNS] Peer expired {} at {}", peer_id, addr);
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received {
                        peer_id,
                        info,
                        ..
                    })) => {
                        println!("[Identify] Identified peer: {} at {:?}", peer_id, info.listen_addrs);
                        info!("[Identify] Identified peer: {} at {:?}", peer_id, info.listen_addrs);
                        
                        // Check if this is bootstrap node (has protocol /p2p-bootstrap)
                        let is_bootstrap = info.protocol_version == "/p2p-bootstrap/0.1.0";
                        if is_bootstrap {
                            println!("[Identify] Connected to bootstrap node: {}", peer_id);
                            info!("[Identify] Connected to bootstrap node: {}", peer_id);
                            bootstrap_peer_id = Some(peer_id);
                            bootstrap_connected = true;
                            
                            // Bootstrap Kademlia routing table with bootstrap node
                            for addr in &info.listen_addrs {
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                            }
                            
                            // Start a Kademlia bootstrap to discover other peers
                            println!("[Kademlia] Starting bootstrap to discover peers...");
                            if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
                                println!("[Kademlia] Bootstrap failed: {:?}", e);
                            }
                            
                        } else if peer_id != local_peer_id && !connected_to_b {
                            // This is a regular node, likely node B
                            println!("[Identify] Discovered node B: {}", peer_id);
                            info!("[Identify] Discovered node B: {}", peer_id);
                            discovered_peers.push(peer_id);
                            
                            // Track this as identified Node B
                            identified_node_b = Some(peer_id);
                            
                            // Add addresses to Kademlia
                            for addr in &info.listen_addrs {
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                            }
                            
                            // Check if already connected to this peer
                            let is_connected = swarm.is_connected(&peer_id);
                            
                            if is_connected {
                                // Already connected, send request immediately
                                connected_to_b = true;
                                println!("[Identify] Already connected to node B, sending request...");
                                
                                let request = P2PRequest {
                                    method: "greet".to_string(),
                                    params: serde_json::json!({
                                        "name": "Node A"
                                    }),
                                };
                                
                                println!("[Request] Sending first P2P request to {}: {:?}", peer_id, request);
                                info!("[Request] Sending first P2P request to {}: {:?}", peer_id, request);
                                let request_id = swarm.behaviour_mut().request_response.send_request(&peer_id, request);
                                println!("[Request] Request sent with ID: {:?}", request_id);
                            } else if let Some(first_addr) = info.listen_addrs.first() {
                                // Not connected yet, dial first
                                let dial_addr = first_addr.clone().with(Protocol::P2p(peer_id));
                                println!("[Identify] Dialing node B at: {}", dial_addr);
                                if let Err(e) = swarm.dial(dial_addr) {
                                    warn!("[Identify] Failed to dial node B: {:?}", e);
                                }
                                // Request will be sent in ConnectionEstablished event
                            }
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Kademlia(event)) => {
                        match event {
                            kad::Event::RoutingUpdated { peer, is_new_peer, addresses, .. } => {
                                println!("[Kademlia] Routing updated: peer={}", peer);
                                if is_new_peer {
                                    println!("[Kademlia]  -> New peer added: {}", peer);
                                    println!("[Kademlia]  -> Addresses: {:?}", addresses);
                                    
                                    // If this is not bootstrap and not ourselves, try to dial
                                    if Some(peer) != bootstrap_peer_id && peer != local_peer_id && !connected_to_b {
                                        if let Some(addr) = addresses.iter().next() {
                                            let dial_addr = addr.clone().with(Protocol::P2p(peer));
                                            println!("[Kademlia] Auto-dialing new peer at: {}", dial_addr);
                                            if let Err(e) = swarm.dial(dial_addr) {
                                                warn!("[Kademlia] Failed to dial new peer: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                            kad::Event::RoutablePeer { peer, address } => {
                                println!("[Kademlia] Routable peer discovered: {} at {}", peer, address);
                                // Try to connect to routable peers (potential node B)
                                if Some(peer) != bootstrap_peer_id && peer != local_peer_id && !connected_to_b {
                                    let dial_addr = address.with(Protocol::P2p(peer));
                                    println!("[Kademlia] Auto-dialing peer at: {}", dial_addr);
                                    if let Err(e) = swarm.dial(dial_addr) {
                                        warn!("[Kademlia] Failed to dial peer: {:?}", e);
                                    }
                                }
                            }
                            kad::Event::UnroutablePeer { peer } => {
                                println!("[Kademlia] Peer {} is unroutable", peer);
                            }
                            kad::Event::OutboundQueryProgressed { result, .. } => {
                                match result {
                                    kad::QueryResult::Bootstrap(Ok(_)) => {
                                        println!("[Kademlia] Bootstrap completed");
                                    }
                                    kad::QueryResult::Bootstrap(Err(e)) => {
                                        println!("[Kademlia] Bootstrap failed: {:?}", e);
                                    }
                                    kad::QueryResult::GetClosestPeers(Ok(result)) => {
                                        println!("[Kademlia] Found {} closest peers", result.peers.len());
                                        for peer in &result.peers {
                                            println!("[Kademlia]  -> Closest peer: {:?}", peer);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                tracing::debug!("[Kademlia] Other event: {:?}", event);
                            }
                        }
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                        println!("[Connection] Connected to peer: {} via {:?}", peer_id, endpoint);
                        info!("[Connection] Connected to peer: {} via {:?}", peer_id, endpoint);
                        
                        // Only send request if:
                        // 1. This peer has been identified as Node B (not bootstrap)
                        // 2. We haven't sent request to Node B yet
                        if identified_node_b == Some(peer_id) && !connected_to_b {
                            connected_to_b = true;
                            
                            let request = P2PRequest {
                                method: "greet".to_string(),
                                params: serde_json::json!({
                                    "name": "Node A"
                                }),
                            };
                            
                            println!("[Request] Sending first P2P request to {}: {:?}", peer_id, request);
                            info!("[Request] Sending first P2P request to {}: {:?}", peer_id, request);
                            let request_id = swarm.behaviour_mut().request_response.send_request(&peer_id, request);
                            println!("[Request] Request sent with ID: {:?}", request_id);
                        }
                    }
                    SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                        println!("[Connection] Disconnected from peer: {} (cause: {:?})", peer_id, cause);
                        // If disconnected from node_b, reset flag to allow reconnection
                        if bootstrap_peer_id.map(|id| id != peer_id).unwrap_or(true) {
                            connected_to_b = false;
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::Message { peer, message, .. }
                    )) => {
                        match message {
                            request_response::Message::Request { .. } => {
                                info!("[Response] Received unexpected request from {}", peer);
                            }
                            request_response::Message::Response { request_id, response } => {
                                println!("[Response] Received P2P response for request {:?} from {}", request_id, peer);
                                println!("[Response] Response data: {:?}", response);
                                info!("[Response] Received P2P response for request {:?} from {}", request_id, peer);
                                
                                if response.success {
                                    println!("✅ Request successful!");
                                    info!("✅ Request successful! Data: {}", 
                                        serde_json::to_string_pretty(&response.data).unwrap_or_default());
                                } else {
                                    println!("❌ Request failed: {:?}", response.data);
                                    warn!("❌ Request failed: {:?}", response.data);
                                }
                                
                                // Wait a bit then send another request
                                tokio::time::sleep(Duration::from_secs(3)).await;
                                
                                let next_request = P2PRequest {
                                    method: "calculate".to_string(),
                                    params: serde_json::json!({
                                        "a": 10.0,
                                        "b": 5.0,
                                        "op": "mul"
                                    }),
                                };
                                
                                println!("[Request] Sending next P2P request to {}: {:?}", peer, next_request);
                                info!("[Request] Sending next P2P request: {:?}", next_request);
                                swarm.behaviour_mut().request_response.send_request(&peer, next_request);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::OutboundFailure { peer, request_id, error, .. }
                    )) => {
                        warn!("[Error] Outbound failure for request {:?} to {}: {:?}", request_id, peer, error);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::InboundFailure { peer, error, .. }
                    )) => {
                        warn!("[Error] Inbound failure from {}: {:?}", peer, error);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Ping(event)) => {
                        match event {
                            ping::Event { peer, result: Ok(rtt), .. } => {
                                tracing::debug!("[Ping] {} rtt: {:?}", peer, rtt);
                            }
                            ping::Event { peer, result: Err(e), .. } => {
                                info!("[Ping] {} failed: {:?}", peer, e);
                            }
                        }
                    }
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("[Swarm] P2P listening on {}", address);
                        println!("[Swarm] Node A multiaddr: {}/p2p/{}", address, swarm.local_peer_id());
                        info!("[Swarm] P2P listening on {}", address);
                    }
                    SwarmEvent::IncomingConnection { send_back_addr, .. } => {
                        println!("[Swarm] Incoming P2P connection from {}", send_back_addr);
                    }
                    other => {
                        tracing::trace!("[Swarm] Other event: {:?}", other);
                    }
                }
            }
            // Periodic discovery attempt via Kademlia
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                if bootstrap_connected && !connected_to_b {
                    println!("[Discovery] Attempting to discover peers via Kademlia...");
                    println!("[Discovery] Current state: bootstrap_peer_id={:?}, identified_node_b={:?}, connected_to_b={}", 
                        bootstrap_peer_id, identified_node_b, connected_to_b);
                    // Try to find peers close to our own peer_id
                    swarm.behaviour_mut().kademlia.get_closest_peers(local_peer_id);
                } else if !bootstrap_connected {
                    println!("[Discovery] Waiting for bootstrap connection...");
                }
            }
        }
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
    request_response: cbor::Behaviour<P2PRequest, P2PResponse>,
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}
