# Libp2p P2P + HTTP Demo

这个项目实现了一个基于 libp2p 的 P2P 通信系统，包含以下组件：

1. **Bootstrap 节点** - 用于节点发现和路由
2. **节点 B** - 运行 P2P 服务和 HTTP 服务器 (axum)
3. **节点 A** - 通过 Bootstrap 节点连接到 B，并发送 P2P 请求

## 架构流程

```
┌──────────┐         ┌──────────┐         ┌──────────┐
│  Node A  │◄───────►│ Bootstrap│◄───────►│  Node B  │
│ (Client) │  P2P    │  Server  │   P2P   │(Server+  │
│          │         │          │         │  HTTP)   │
└────┬─────┘         └──────────┘         └────┬─────┘
     │                                          │
     │  1. A 通过 Bootstrap 发现 B              │
     │  2. A 发送 P2P 请求给 B                  │
     │  3. B 调用本地 HTTP API                  │
     │  4. B 返回 HTTP 结果给 A (via P2P)       │
     └──────────────────────────────────────────┘
```

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
Bootstrap Node PeerId: 12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
Listening on /ip4/0.0.0.0/tcp/9090
Listening on /ip4/127.0.0.1/tcp/9090/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
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

## 注意事项

1. Bootstrap 节点使用固定的 secret_key_seed 来生成确定性的 PeerId
2. 节点 A 和 B 使用随机生成的身份
3. 确保所有节点都能访问彼此的端口
4. 默认监听 `0.0.0.0`，可以从其他机器连接
