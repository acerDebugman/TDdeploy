# Libp2p P2P + HTTP Demo

这个项目实现了一个基于 libp2p 的 P2P 通信系统，包含以下组件：

1. **Bootstrap 节点** - 用于节点发现和路由，支持 **AutoNAT** 和 **Relay** 中继功能
2. **节点 B** - 运行 P2P 服务和 HTTP 服务器 (axum)
3. **节点 A** - 通过 Bootstrap 节点连接到 B，并发送 P2P 请求

## 功能特性

- ✅ **Kademlia DHT** - 分布式节点发现
- ✅ **Identify Protocol** - 节点信息交换
- ✅ **Request-Response** - P2P 请求响应
- ✅ **Relay Server** - 中继转发（用于 NAT 穿透）
- ✅ **AutoNAT** - 自动 NAT 检测
- ✅ **HTTP API** - axum 实现的 REST API
- ✅ **TCP/QUIC** 双协议支持

## 架构流程

```
┌──────────┐         ┌─────────────────────────┐         ┌──────────┐
│  Node A  │◄───────►│       Bootstrap         │◄───────►│  Node B  │
│ (Client) │  P2P    │       Server            │   P2P   │(Server+  │
│          │         │  ┌─────────────────┐    │         │  HTTP)   │
└────┬─────┘         │  │  Features:      │    │         └────┬─────┘
     │               │  │  - Kademlia     │    │              │
     │               │  │  - AutoNAT      │    │              │
     │               │  │  - Relay        │    │              │
     │               │  │  - Identify     │    │              │
     │               │  └─────────────────┘    │              │
     │               └─────────────────────────┘              │
     │                                                        │
     │  1. A 通过 Bootstrap 发现 B                            │
     │  2. A 发送 P2P 请求给 B                                │
     │  3. B 调用本地 HTTP API                                │
     │  4. B 返回 HTTP 结果给 A (via P2P)                     │
     └────────────────────────────────────────────────────────┘
```

### Relay 中继功能

Bootstrap 节点作为 **Relay Server**，帮助位于 NAT 后的节点建立连接：

```
Node A (NAT后) ───► Bootstrap Relay ◄─── Node B (NAT后)
                        │
                        └── 转发流量
```

连接字符串格式：
```
/ip4/<relay_ip>/tcp/<port>/p2p/<relay_peer_id>/p2p-circuit/p2p/<target_peer_id>
```

### AutoNAT 功能

Bootstrap 节点作为 **AutoNAT Server**，帮助其他节点检测自己的 NAT 状态：
- **Public**: 节点可直接被公网访问
- **Private**: 节点位于 NAT 后，需要通过 Relay 中转

## 编译

```bash
cargo build --release
```

生成的二进制文件位于 `target/release/`：
- `bootstrap` - Bootstrap 节点
- `node_b` - 节点 B (P2P + HTTP 服务器)
- `node_a` - 节点 A (P2P 客户端)

## 使用方法

### 1. 启动 Bootstrap 节点

```bash
./target/release/bootstrap --port 9090 --secret-key-seed 0
```

输出示例：
```
=== Bootstrap Node Started ===
PeerId: 12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
TCP Port: 9090
QUIC Port: 9090

Features enabled:
  - Kademlia DHT (node discovery)
  - Relay Server (circuit relay)
  - AutoNAT Server (NAT detection)
  - Identify Protocol
  - Ping Protocol

Connection strings:
  TCP:  /ip4/<ip>/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
  QUIC: /ip4/<ip>/udp/9090/quic-v1/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
```

**记录下这个 PeerId，稍后需要用到！**

### 2. 启动节点 B

```bash
./target/release/node_b 10001 8080 "/ip4/127.0.0.1/tcp/9090/p2p/<BOOTSTRAP_PEER_ID>"
```

参数：
- `10001` - P2P 监听端口
- `8080` - HTTP 服务器端口
- `/ip4/...` - Bootstrap 节点地址

输出示例：
```
Local PeerId: 12D3KooWXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Connecting to bootstrap node: /ip4/127.0.0.1/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
HTTP server listening on http://0.0.0.0:8080
P2P listening on /ip4/0.0.0.0/tcp/10001
Node B multiaddr: /ip4/0.0.0.0/tcp/10001/p2p/12D3KooWXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

**记录下节点 B 的 PeerId！**

### 3. 启动节点 A

```bash
./target/release/node_a 10002 "/ip4/127.0.0.1/tcp/9090/p2p/<BOOTSTRAP_PEER_ID>"
```

参数：
- `10002` - P2P 监听端口
- `/ip4/...` - Bootstrap 节点地址

### 4. 观察通信

节点 A 启动后会：
1. 连接到 Bootstrap 节点
2. 发现节点 B
3. 自动连接到节点 B
4. 发送 P2P 请求 (`greet` 方法)
5. 收到节点 B 的响应
6. 发送下一个请求 (`calculate` 方法)
7. 循环往复

## HTTP API (节点 B)

节点 B 提供以下 HTTP 接口：

### POST /api/action

请求体：
```json
{
  "action": "greet",
  "data": {
    "name": "World"
  }
}
```

响应：
```json
{
  "success": true,
  "message": "Hello, World!",
  "result": {
    "greeting": "Hello, World!",
    "timestamp": "2024-01-01T00:00:00+00:00"
  }
}
```

支持的 action：
- `greet` - 问候语，接受 `name` 参数
- `calculate` - 计算器，接受 `a`, `b`, `op` (add/sub/mul/div) 参数
- `echo` - 回显，返回传入的 data

示例：
```bash
curl -X POST http://localhost:8080/api/action \
  -H "Content-Type: application/json" \
  -d '{"action": "greet", "data": {"name": "Alice"}}'

curl -X POST http://localhost:8080/api/action \
  -H "Content-Type: application/json" \
  -d '{"action": "calculate", "data": {"a": 10, "b": 5, "op": "mul"}}'
```

## P2P 协议

### 协议名称
`/p2p-api/1.0.0`

### 请求格式 (CBOR)
```rust
struct P2PRequest {
    method: String,       // "greet", "calculate", "echo"
    params: serde_json::Value,
}
```

### 响应格式 (CBOR)
```rust
struct P2PResponse {
    success: bool,
    data: serde_json::Value,
}
```

## 完整的启动脚本示例

```bash
#!/bin/bash

# 1. 启动 Bootstrap 节点 (终端 1)
./target/release/bootstrap --port 9090 --secret-key-seed 0

# 2. 等待获取 Bootstrap PeerId，然后启动节点 B (终端 2)
# 假设 Bootstrap PeerId = 12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
./target/release/node_b 10001 8080 "/ip4/127.0.0.1/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN"

# 3. 等待节点 B 启动完成，然后启动节点 A (终端 3)
./target/release/node_a 10002 "/ip4/127.0.0.1/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN"
```

## 使用 Relay 中继连接（NAT 穿透）

如果节点 A 和 B 都位于 NAT 后，可以通过 Bootstrap 节点的 Relay 功能连接：

### 1. 节点 B 通过 Relay 连接到 Bootstrap

```bash
# 节点 B 先连接到 Bootstrap
./target/release/node_b 10001 8080 "/ip4/<bootstrap_ip>/tcp/9090/p2p/<bootstrap_peer_id>"
```

### 2. 节点 A 通过 Relay 连接到节点 B

```bash
# 使用 Relay 地址连接
RELAY_ADDR="/ip4/<bootstrap_ip>/tcp/9090/p2p/<bootstrap_peer_id>/p2p-circuit/p2p/<node_b_peer_id>"
./target/release/node_a 10002 "/ip4/<bootstrap_ip>/tcp/9090/p2p/<bootstrap_peer_id>" "$RELAY_ADDR"
```

## Bootstrap 节点日志说明

Bootstrap 节点会输出详细的协议日志：

```
[Identify] Peer xxx identified, protocol version: "/p2p-node/0.1.0"
[Kademlia] Routing updated: peer=xxx
[Relay] New reservation accepted from xxx          # 有新的节点申请 Relay 中继槽位
[Relay] Circuit established: xxx -> yyy            # 建立了中继电路
[AutoNAT] Inbound probe request from xxx           # 收到 NAT 探测请求
[Swarm] Connected to xxx via Dialer { ... }
```

## 注意事项

1. **Bootstrap 节点**使用固定的 `secret_key_seed` 来生成确定性的 PeerId
2. **节点 A 和 B**使用随机生成的身份
3. 确保所有节点都能访问彼此的端口
4. 默认监听 `0.0.0.0`，可以从其他机器连接
5. **Relay 功能**：Bootstrap 节点默认接受 Relay 预留请求，帮助 NAT 后的节点建立连接
6. **AutoNAT 功能**：Bootstrap 节点作为 AutoNAT 服务器，帮助其他节点检测 NAT 状态
7. 如果节点位于不同网络，需要确保防火墙允许 P2P 端口通信

## 协议支持

| 协议 | Bootstrap | Node A | Node B |
|------|-----------|--------|--------|
| Kademlia DHT | ✅ Server | ✅ Client | ✅ Client |
| Relay | ✅ Server | ❌ | ❌ |
| AutoNAT | ✅ Server | ❌ | ❌ |
| Identify | ✅ | ✅ | ✅ |
| Ping | ✅ | ✅ | ✅ |
| Request-Response | ❌ | ✅ Client | ✅ Server |
| HTTP API | ❌ | ❌ | ✅ (axum) |
