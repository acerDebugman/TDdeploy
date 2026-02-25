use std::error::Error;
use std::time::Duration;

use futures::StreamExt;
use libp2p::{
    core::multiaddr::Protocol,
    identify, identity, kad, noise, request_response::{self, cbor, ProtocolSupport},
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
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let p2p_port = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10002u16);
    let bootstrap_addr = args.get(2).cloned();
    let target_peer_id = args.get(3).cloned();

    info!("Starting Node A - P2P port: {}", p2p_port);

    // Create identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();
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
    if let Some(addr) = bootstrap_addr.clone() {
        let bootstrap_multiaddr: Multiaddr = addr.parse()?;
        info!("Connecting to bootstrap node: {}", bootstrap_multiaddr);
        swarm.dial(bootstrap_multiaddr)?;
    }

    // Store discovered peers
    let mut discovered_peers: Vec<PeerId> = Vec::new();
    let mut connected_to_b = false;

    // Wait for bootstrap connection and discovery
    if bootstrap_addr.is_some() && target_peer_id.is_none() {
        info!("Waiting to discover peers via bootstrap...");
    }

    // Main P2P loop
    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received {
                        peer_id,
                        info,
                        ..
                    })) => {
                        info!("Identified peer: {} at {:?}", peer_id, info.listen_addrs);
                        
                        // Check if this is bootstrap node (has protocol /p2p-bootstrap)
                        let is_bootstrap = info.protocol_version == "/p2p-bootstrap/0.1.0";
                        if is_bootstrap {
                            info!("Connected to bootstrap node: {}", peer_id);
                            bootstrap_peer_id = Some(peer_id);
                        } else if peer_id != local_peer_id && !connected_to_b {
                            // This is a regular node, likely node B
                            info!("Discovered node B: {}", peer_id);
                            discovered_peers.push(peer_id);
                            
                            // Add addresses to Kademlia
                            for addr in &info.listen_addrs {
                                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                            }
                            
                            // Connect to this peer if it's not already connected
                            if let Some(first_addr) = info.listen_addrs.first() {
                                let dial_addr = first_addr.clone().with(Protocol::P2p(peer_id));
                                info!("Dialing node B at: {}", dial_addr);
                                if let Err(e) = swarm.dial(dial_addr) {
                                    warn!("Failed to dial node B: {:?}", e);
                                }
                            }
                        }
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                        info!("Connected to peer: {} via {:?}", peer_id, endpoint);
                        
                        // Check if this is node B (not bootstrap)
                        if bootstrap_peer_id.map(|id| id != peer_id).unwrap_or(true) && !connected_to_b {
                            connected_to_b = true;
                            
                            // Send a test request to node B
                            let request = P2PRequest {
                                method: "greet".to_string(),
                                params: serde_json::json!({
                                    "name": "Node A"
                                }),
                            };
                            
                            info!("Sending P2P request to {}: {:?}", peer_id, request);
                            let request_id = swarm.behaviour_mut().request_response.send_request(&peer_id, request);
                            info!("Request sent with ID: {:?}", request_id);
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::Message { peer, message, .. }
                    )) => {
                        match message {
                            request_response::Message::Request { .. } => {
                                info!("Received unexpected request from {}", peer);
                            }
                            request_response::Message::Response { request_id, response } => {
                                info!("Received P2P response for request {:?} from {}", request_id, peer);
                                info!("Response: {:?}", response);
                                
                                if response.success {
                                    info!("✅ Request successful! Data: {}", 
                                        serde_json::to_string_pretty(&response.data).unwrap_or_default());
                                } else {
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
                                
                                info!("Sending next P2P request: {:?}", next_request);
                                swarm.behaviour_mut().request_response.send_request(&peer, next_request);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::OutboundFailure { peer, request_id, error, .. }
                    )) => {
                        warn!("Outbound failure for request {:?} to {}: {:?}", request_id, peer, error);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::InboundFailure { peer, error, .. }
                    )) => {
                        warn!("Inbound failure from {}: {:?}", peer, error);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Kademlia(event)) => {
                        if let kad::Event::RoutingUpdated { peer, .. } = &event {
                            info!("Kademlia routing updated: peer={}", peer);
                        }
                    }
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("P2P listening on {}", address);
                        info!("Node A is ready. Waiting for peers...");
                    }
                    SwarmEvent::IncomingConnection { send_back_addr, .. } => {
                        info!("Incoming P2P connection from {}", send_back_addr);
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                // Try to dial a specific peer if provided
                if let Some(ref target_id_str) = target_peer_id {
                    if let Ok(peer_id) = target_id_str.parse::<PeerId>() {
                        if !connected_to_b {
                            // Build multiaddr from bootstrap info
                            if let Some(ref bootstrap) = bootstrap_addr {
                                // Parse bootstrap address and modify for target
                                let parts: Vec<&str> = bootstrap.split("/p2p/").collect();
                                if parts.len() >= 1 {
                                    let base_addr = parts[0];
                                    let target_addr = format!("{}/p2p/{}", base_addr, peer_id);
                                    info!("Attempting to dial target: {}", target_addr);
                                    if let Ok(addr) = target_addr.parse::<Multiaddr>() {
                                        let _ = swarm.dial(addr);
                                    }
                                }
                            }
                        }
                    }
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
}
