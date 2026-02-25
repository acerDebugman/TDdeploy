use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{response::Json, routing::post, Router};
use futures::StreamExt;
use libp2p::{
    core::multiaddr::Protocol,
    identify, identity, kad, mdns, noise, ping, request_response::{self, cbor, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, StreamProtocol,
};
use serde::{Deserialize, Serialize};
use tracing::{info};
use tracing_subscriber::EnvFilter;

// HTTP API request/response types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpRequest {
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpResponse {
    pub success: bool,
    pub message: String,
    pub result: Option<serde_json::Value>,
}

// P2P request/response types
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

// HTTP client for calling local API

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
        .unwrap_or(10001u16);
    let http_port = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080u16);
    let bootstrap_addr = args.get(3).cloned();

    println!("Starting Node B - P2P port: {}, HTTP port: {}", p2p_port, http_port);
    info!("Starting Node B - P2P port: {}, HTTP port: {}", p2p_port, http_port);

    // Create identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();
    println!("Local PeerId: {}", local_peer_id);
    info!("Local PeerId: {}", local_peer_id);

    // Setup request-response protocol - use string protocol name
    let protocols = std::iter::once((StreamProtocol::new("/p2p-api/1.0.0"), ProtocolSupport::Full));
    let cfg = request_response::Config::default();
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
    if let Some(addr) = bootstrap_addr.clone() {
        let bootstrap_multiaddr: Multiaddr = addr.parse()?;
        println!("Connecting to bootstrap node: {}", bootstrap_multiaddr);
        info!("Connecting to bootstrap node: {}", bootstrap_multiaddr);
        swarm.dial(bootstrap_multiaddr)?;
    } else {
        println!("Warning: No bootstrap address provided");
    }

    // Start HTTP server
    let http_app = Router::new()
        .route("/api/action", post(handle_http_action));

    let http_addr = SocketAddr::from(([0, 0, 0, 0], http_port));
    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;
    println!("HTTP server listening on http://{}", http_addr);
    info!("HTTP server listening on http://{}", http_addr);

    tokio::spawn(async move {
        axum::serve(http_listener, http_app).await.unwrap();
    });

    // Main P2P loop
    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::Message { peer, message, .. }
                    )) => {
                        match message {
                            request_response::Message::Request { request, channel, .. } => {
                                info!("Received P2P request from {}: {:?}", peer, request);
                                
                                // Call local HTTP API synchronously
                                let _http_request = HttpRequest {
                                    action: request.method.clone(),
                                    data: request.params.clone(),
                                };
                                
                                let _http_url = format!("http://127.0.0.1:{}/api/action", http_port);
                                
                                // Create HTTP client and call local API
                                let client = reqwest::Client::new();
                                
                                let response = match tokio::task::block_in_place(|| {
                                    tokio::runtime::Handle::current().block_on(async {
                                        client.post(&_http_url).json(&_http_request).send().await
                                    })
                                }) {
                                    Ok(resp) => match tokio::task::block_in_place(|| {
                                        tokio::runtime::Handle::current().block_on(async {
                                            resp.json::<HttpResponse>().await
                                        })
                                    }) {
                                        Ok(http_resp) => P2PResponse {
                                            success: http_resp.success,
                                            data: http_resp.result.unwrap_or(serde_json::json!({
                                                "message": http_resp.message
                                            })),
                                        },
                                        Err(e) => P2PResponse {
                                            success: false,
                                            data: serde_json::json!({
                                                "error": format!("Failed to parse HTTP response: {}", e)
                                            }),
                                        },
                                    },
                                    Err(e) => P2PResponse {
                                        success: false,
                                        data: serde_json::json!({
                                            "error": format!("HTTP request failed: {}", e)
                                        }),
                                    },
                                };
                                
                                if let Err(e) = swarm.behaviour_mut().request_response.send_response(channel, response.clone()) {
                                    info!("Failed to send response: {:?}", e);
                                } else {
                                    info!("Sent P2P response to {}: {:?}", peer, response);
                                }
                            }
                            request_response::Message::Response { response, .. } => {
                                info!("Received P2P response: {:?}", response);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(
                        request_response::Event::InboundFailure { peer, error, .. }
                    )) => {
                        info!("Inbound failure from {}: {:?}", peer, error);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received {
                        peer_id,
                        info,
                        ..
                    })) => {
                        info!("Identified peer: {} at {:?}", peer_id, info.listen_addrs);
                        for addr in &info.listen_addrs {
                            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Kademlia(event)) => {
                        info!("Kademlia event: {:?}", event);
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                        for (peer_id, addr) in peers {
                            info!("[mDNS] Discovered peer {} at {}", peer_id, addr);
                            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                        for (peer_id, addr) in peers {
                            info!("[mDNS] Peer expired {} at {}", peer_id, addr);
                        }
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Ping(event)) => {
                        match event {
                            ping::Event { peer, result: Ok(rtt), .. } => {
                                tracing::debug!("Ping {} rtt: {:?}", peer, rtt);
                            }
                            ping::Event { peer, result: Err(e), .. } => {
                                info!("Ping {} failed: {:?}", peer, e);
                            }
                        }
                    }
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("P2P listening on {}", address);
                        info!("Node B multiaddr: {}/p2p/{}", address, swarm.local_peer_id());
                    }
                    SwarmEvent::IncomingConnection { send_back_addr, .. } => {
                        info!("Incoming P2P connection from {}", send_back_addr);
                    }
                    _ => {}
                }
            }
        }
    }
}

// HTTP handler
async fn handle_http_action(Json(request): Json<HttpRequest>) -> Json<HttpResponse> {
    info!("HTTP API received request: {:?}", request);

    // Process the request
    let result = match request.action.as_str() {
        "greet" => {
            let name = request
                .data
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("World");
            HttpResponse {
                success: true,
                message: format!("Hello, {}!", name),
                result: Some(serde_json::json!({
                    "greeting": format!("Hello, {}!", name),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
            }
        }
        "calculate" => {
            let a = request.data.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let b = request.data.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let op = request
                .data
                .get("op")
                .and_then(|v| v.as_str())
                .unwrap_or("add");
            
            let result = match op {
                "add" => a + b,
                "sub" => a - b,
                "mul" => a * b,
                "div" => if b != 0.0 { a / b } else { 0.0 },
                _ => a + b,
            };
            
            HttpResponse {
                success: true,
                message: "Calculation completed".to_string(),
                result: Some(serde_json::json!({
                    "result": result,
                    "operation": op,
                    "operands": [a, b]
                })),
            }
        }
        "echo" => HttpResponse {
            success: true,
            message: "Echo".to_string(),
            result: Some(request.data),
        },
        _ => HttpResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
            result: None,
        },
    };

    Json(result)
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
    request_response: cbor::Behaviour<P2PRequest, P2PResponse>,
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}
