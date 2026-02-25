# Libp2p P2P + HTTP Demo

这个项目实现了一个基于 libp2p 的 P2P 通信系统，包含以下组件：

1. **Bootstrap 节点** - 用于节点发现和路由，支持 **AutoNAT** 和 **Relay** 中继功能
2. **节点 B** - 运行 P2P 服务和 HTTP 服务器 (axum)
3. **节点 A** - 通过多种方式发现 Node B，并发送 P2P 请求

## 功能特性

- ✅ **Kademlia DHT** - 分布式节点发现
- ✅ **mDNS** - 局域网自动发现（同一网络内无需 bootstrap）
- ✅ **Identify Protocol** - 节点信息交换
- ✅ **Request-Response** - P2P 请求响应
- ✅ **Relay Server** - 中继转发（用于 NAT 穿透）
- ✅ **AutoNAT** - 自动 NAT 检测
- ✅ **HTTP API** - axum 实现的 REST API
- ✅ **Ping Protocol** - 节点存活检测
- ✅ **TCP/QUIC** 双协议支持

## 架构流程

```
┌─────────────────────────────────────────────────────────────────────┐
│                         节点发现方式                                  │
├─────────────────────────────────────────────────────────────────────┤
│  1. mDNS (同一局域网)   2. Kademlia (通过 Bootstrap)   3. 直接拨号    │
│                                                                     │
│   Node A ◄────────► Node B         Node A ◄────► Bootstrap         │
│   (自动发现)                        (DHT 路由表)   ▲                │
│                                                    │                │
│                                                    ▼                │
│                                                  Node B             │
└─────────────────────────────────────────────────────────────────────┘

详细通信流程：
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
     │  1. Node A 通过以下方式发现 Node B:                     │
     │     a) mDNS: 同一局域网自动发现                         │
     │     b) Kademlia: 通过 Bootstrap 的 DHT 发现             │
     │     c) 直接拨号: 通过命令行参数指定地址                 │
     │                                                        │
     │  2. Node A 连接 Node B 并建立 P2P 连接                  │
     │  3. Node A 发送 P2P request (greet) 给 Node B           │
     │  4. Node B 收到请求，调用本地 HTTP API                  │
     │  5. Node B 返回 HTTP 结果给 Node A (via P2P response)   │
     │  6. Node A 收到响应，等待 3 秒后发送下一个请求          │
     └────────────────────────────────────────────────────────┘
```

### Node A 发现 Node B 的三种方式

#### 方式 1: mDNS 自动发现（推荐，同一局域网）
在同一局域网内，Node A 和 Node B 可以通过 mDNS 自动发现彼此，**无需 Bootstrap 节点**。

```
Node A ──mDNS──► 局域网 ◄──mDNS── Node B
```

#### 方式 2: 通过 Bootstrap + Kademlia
当节点位于不同网络时，通过 Bootstrap 节点进行发现：

```
Node A ──► Bootstrap ◄── Node B
   │           │
   └───────────┘ (Kademlia DHT 交换节点信息)
```

#### 方式 3: 直接拨号
通过命令行参数直接指定 Node B 的地址：

```
Node A ─────────────► Node B
      (直接 TCP/QUIC 连接)
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

### 快速开始（同一台机器，mDNS 自动发现）

在同一台机器上测试时，最简单的方式是使用 **mDNS 自动发现**，无需启动 Bootstrap 节点：

```bash
# 终端 1: 启动节点 B（不需要 bootstrap 地址）
./target/release/node_b 10001 8080

# 终端 2: 启动节点 A（不需要 bootstrap 地址）
./target/release/node_a 10002
```

Node A 会自动通过 mDNS 发现 Node B 并建立连接。

### 标准流程（使用 Bootstrap）

#### 1. 启动 Bootstrap 节点

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

#### 2. 启动节点 B

```bash
./target/release/node_b 10001 8080 "/ip4/127.0.0.1/tcp/9090/p2p/<BOOTSTRAP_PEER_ID>"
```

参数：
- `10001` - P2P 监听端口
- `8080` - HTTP 服务器端口
- `/ip4/...` - Bootstrap 节点地址（可选，同一局域网可不传）

输出示例：
```
Starting Node B - P2P port: 10001, HTTP port: 8080
Local PeerId: 12D3KooWXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
Connecting to bootstrap node: /ip4/127.0.0.1/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
HTTP server listening on http://0.0.0.0:8080
P2P listening on /ip4/0.0.0.0/tcp/10001
Node B multiaddr: /ip4/0.0.0.0/tcp/10001/p2p/12D3KooWXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[mDNS] Discovered peer ...
```

**记录下节点 B 的 PeerId！**

#### 3. 启动节点 A

**方式 A: 通过 Bootstrap 自动发现（推荐）**
```bash
./target/release/node_a 10002 "/ip4/127.0.0.1/tcp/9090/p2p/<BOOTSTRAP_PEER_ID>"
```

**方式 B: 直接拨号到 Node B**
```bash
./target/release/node_a 10002 "/ip4/127.0.0.1/tcp/9090/p2p/<BOOTSTRAP_PEER_ID>" "/ip4/127.0.0.1/tcp/10001/p2p/<NODE_B_PEER_ID>"
```

参数：
- `10002` - P2P 监听端口
- 第 2 个参数 - Bootstrap 节点地址（可选）
- 第 3 个参数 - Node B 直接地址（可选）

### 4. 观察通信

Node A 启动后的行为：

1. **连接到 Bootstrap**（如果提供了地址）
2. **通过 mDNS 发现**同一局域网的节点
3. **通过 Kademlia 发现**其他已连接到 Bootstrap 的节点
4. **直接拨号**（如果提供了 Node B 地址）
5. **连接到 Node B** 后发送第一个 P2P 请求 (`greet` 方法)
6. **收到响应**后，等待 3 秒发送下一个请求 (`calculate`)
7. **循环往复**

典型日志输出（Node A）：
```
Starting Node A - P2P port: 10002
Local PeerId: 12D3KooW...
[mDNS] Discovered peer 12D3KooW... at /ip4/192.168.1.100/tcp/10001
[mDNS] Dialing discovered peer at /ip4/192.168.1.100/tcp/10001/p2p/12D3KooW...
[Connection] Connected to peer: 12D3KooW... via ...
[Request] Sending first P2P request to 12D3KooW...: P2PRequest { method: "greet", ... }
[Response] Received P2P response for request ... from 12D3KooW...
✅ Request successful!
[Request] Sending next P2P request: P2PRequest { method: "calculate", ... }
```

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

### 场景 1: 同一台机器测试（mDNS 自动发现，最简单）

```bash
#!/bin/bash

# 终端 1: 启动节点 B（不需要 bootstrap）
./target/release/node_b 10001 8080

# 终端 2: 启动节点 A（不需要 bootstrap，自动通过 mDNS 发现 B）
./target/release/node_a 10002
```

### 场景 2: 使用 Bootstrap（跨网络）

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

### 场景 3: 直接拨号（无需 bootstrap，需要知道 Node B 地址）

```bash
# 终端 1: 启动节点 B
./target/release/node_b 10001 8080

# 终端 2: 启动节点 A，直接拨号到 Node B
# 参数：p2p_port bootstrap_addr(可选) node_b_addr(直接)
./target/release/node_a 10002 "" "/ip4/127.0.0.1/tcp/10001/p2p/<NODE_B_PEER_ID>"
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

## 节点日志说明

### Bootstrap 节点日志

```
[Identify] Peer xxx identified, protocol version: "/p2p-node/0.1.0"
[Kademlia] Routing updated: peer=xxx
[Relay] New reservation accepted from xxx          # 有新的节点申请 Relay 中继槽位
[Relay] Circuit established: xxx -> yyy            # 建立了中继电路
[AutoNAT] Inbound probe request from xxx           # 收到 NAT 探测请求
[Swarm] Connected to xxx via Dialer { ... }
```

### Node A 日志

```
[mDNS] Discovered peer xxx at ...                  # mDNS 发现节点
[Identify] Identified peer: xxx                    # Identify 协议识别节点
[Kademlia] Routable peer discovered: xxx           # Kademlia 发现可路由节点
[Connection] Connected to peer: xxx                # 成功建立连接
[Request] Sending first P2P request to xxx         # 发送第一个请求
[Response] Received P2P response                   # 收到响应
```

### Node B 日志

```
HTTP server listening on http://0.0.0.0:8080       # HTTP 服务启动
P2P listening on /ip4/0.0.0.0/tcp/10001            # P2P 服务启动
[mDNS] Discovered peer xxx                         # 发现其他节点
[Request] Received P2P request from xxx            # 收到 P2P 请求
[HTTP] Calling local HTTP API                      # 调用本地 HTTP API
[Response] Sending P2P response to xxx             # 发送响应
```

## 注意事项

1. **Bootstrap 节点**使用固定的 `secret_key_seed` 来生成确定性的 PeerId
2. **节点 A 和 B**使用随机生成的身份（每次启动都不同）
3. **mDNS 自动发现**仅在同一局域网内有效
4. 默认监听 `0.0.0.0`，可以从其他机器连接
5. **Relay 功能**：Bootstrap 节点默认接受 Relay 预留请求，帮助 NAT 后的节点建立连接
6. **AutoNAT 功能**：Bootstrap 节点作为 AutoNAT 服务器，帮助其他节点检测 NAT 状态
7. 如果节点位于不同网络，需要确保防火墙允许 P2P 端口通信

## 协议支持

| 协议 | Bootstrap | Node A | Node B |
|------|-----------|--------|--------|
| Kademlia DHT | ✅ Server | ✅ Client | ✅ Client |
| mDNS | ❌ | ✅ | ✅ |
| Relay | ✅ Server | ❌ | ❌ |
| AutoNAT | ✅ Server | ❌ | ❌ |
| Identify | ✅ | ✅ | ✅ |
| Ping | ✅ | ✅ | ✅ |
| Request-Response | ❌ | ✅ Client | ✅ Server |
| HTTP API | ❌ | ❌ | ✅ (axum) |

## 故障排查

### 问题：Node A 无法发现 Node B

**检查清单：**
1. 如果是同一机器：确认两个节点都启动了，mDNS 会自动发现
2. 如果是不同机器：确认 Bootstrap 节点已启动，且 Node B 已连接到 Bootstrap
3. 检查防火墙设置，确保端口可以通信
4. 查看日志中的 `[mDNS]`、`[Kademlia]`、`[Identify]` 相关日志

### 问题：Ping 失败报错

确保所有节点都支持 Ping 协议（已实现，无需额外配置）。

### 问题：无法编译

```bash
# 清理并重新编译
cargo clean
cargo build --release
```
