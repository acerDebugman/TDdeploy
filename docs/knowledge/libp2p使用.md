## 基础认知

所以：libp2p 是构建在传统 tcp/udp 上的协议，并且基于 tcp/udp 上，自己构建了几层网络：传输层 + 





可靠上讲，至少2个公网节点：

1. bootstrap 节点
2. circuit relay 节点

两个节点当然可以放到一台服务器上。





如果你部署私有 libp2p 网络：

1. **必须准备**：1-2 个公网节点作为 **Bootstrap 节点**（用于地址发现，轻量级）
2. **可选准备**：1 个公网节点开启 **Circuit Relay**（作为 fallback，仅当直连失败时启用）
3. **优化策略**：
   - 优先使用 **QUIC + Hole Punching**（成功率比 TCP 高）
   - 在移动网络（4G/5G）上，对称 NAT 更常见，Relay 几乎是必须的





NOISE： 加密协议

Yamux： 链接上的数据流

```
let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?;
```



swarm.select_next_som() 得到的 event， 其实应该是要映射到每个自定义的 Behavior 的内部的 event 的，不然没法知道某个消息是属于哪一个模块的！

```
 loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received {
                        peer_id,
                        info,
                        ..
```







## 核心概念

```
### 1. Peer ID（节点身份）

- **本质**：公钥的哈希值（如 `QmY3...` 或 `12D3...`），全局唯一且不可伪造
- **作用**：替代 IP 地址成为网络中的"身份证号"，IP 变化不影响身份识别
- **类比**：就像手机号（Peer ID） vs 家庭住址（IP 地址），搬家（换 IP）号码不变

### 2. Private Key（身份密钥）

- 每个节点启动时生成密钥对（通常 Ed25519），用于：
  - 证明"我是这个 Peer ID 的拥有者"
  - 加密通信（Noise 协议握手）

### 3. Multiaddr（多地址）

- **格式**：自描述的协议栈地址，如 `/ip4/192.168.1.5/tcp/4001/p2p/QmPeerId`
- **优势**：支持嵌套（TCP 上的 TLS 上的 WebSocket），明确表达"如何连接"
- **对比**：传统 `192.168.1.5:4001` 无法表达传输协议和加密方式

### 4. Listen Addrs vs External Addrs（监听地址 vs 外部地址）

- **Listen**：节点实际绑定的网卡地址（如 `0.0.0.0:4001`）
- **External**：经过 NAT 映射后的公网可见地址（通过 AutoNAT 协议自动探测）

### 5. Transport（传输适配器）

- **抽象接口**：统一封装 TCP、QUIC、WebSocket、WebRTC 等不同传输协议

- **Upgrading（升级机制）**：连接建立后自动叠加安全层（Noise）和多路复用层（Yamux）

```
  TCP 连接 → Noise 加密通道 → Yamux 多路复用器 → 应用数据流
  ```

  

### 6. Swarm（蜂群/网络核心）

- **角色**：管理所有连接、协议协商、事件调度的中央控制器
- **类比**：像是手机的"基带芯片+操作系统网络栈"，处理：
  - 维护与多个节点的连接池
  - 根据 Peer ID 路由数据到对应连接
  - 处理连接打开/关闭/错误事件

## 四、通信层：如何对话？

### 7. Protocol（应用协议）

- **定义**：自定义通信规则，如 `/chat/1.0.0`、`/file-transfer/2.0`
- **Multistream-select（协议协商）**：
  - 连接建立后，双方交换支持的协议列表
  - 自动匹配共同支持的协议版本（如协商使用 `/chat/1.0` 而非 `/chat/2.0`）

### 8. Stream（流/逻辑通道）

- **概念**：在单一物理连接上虚拟出的独立双向数据流
- **多路复用**：一个 TCP 连接可同时承载多个 Stream（如：A 流传文件，B 流传聊天消息，互不阻塞）
- **对比**：HTTP/1.1 一个连接一个请求；libp2p 一个连接无限个并发 Stream

## 五、网络层：如何发现和路由？

### 9. Peer Routing（节点路由）

- **Kademlia DHT**：分布式哈希表，存储 `Peer ID → Multiaddr` 的映射
- **作用**：知道对方 ID 但不知道 IP 时，通过 DHT 查询其当前地址（类似去中心化 DNS）

### 10. Content Routing（内容路由）

- **概念**：通过内容哈希（CID）而非节点地址定位数据
- **机制**：DHT 存储 `CID → 拥有该内容的 Peer ID 列表`
- **应用**：IPFS 文件查找、BitTorrent 的 Magnet 链接

### 11. PubSub（发布订阅）

- **Gossipsub 协议**：消息广播网络，适合实时通信（聊天、通知）
- **特点**：
  - **Mesh（网状网络）**：节点间形成转发网格，消息像病毒一样传播
  - **Topic（主题）**：订阅 `/sports` 主题即可接收所有相关消息，无需知道发送者是谁

## 六、辅助机制：应对现实网络

### 12. NAT Traversal（NAT 穿透）

- **AutoNAT**：自动探测本机是否处于 NAT 后
- **Hole Punching（打洞）**：协调双方同时发包，在防火墙上"撞开"通道实现直连
- **Circuit Relay（中继）**：打洞失败时的 fallback，通过公网节点转发流量（但加密保护隐私）

### 13. Connection Gating（连接门禁）

- 安全机制：允许开发者定义"允许/拒绝"哪些节点连接（基于 IP 黑名单、Peer ID 白名单等）
  ```





## 测试用例

1. 测试 ping
2. relay server + ping + axum http server 结合测试
   1. 使用 libp2p + relay server 进行 http 协议的收发





relay-server 测试环境：

relay_server:

```
(base) algo@algo-PC:~/rust_space/TDdeploy/coding_tips/tutor/test_libp2p$ cargo run --bin relay_server -- --secret-key-seed 42 --port 9933
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `/home/algo/rust_space/TDdeploy/target/debug/relay_server --secret-key-seed 42 --port 9933`
Listening on /ip4/127.0.0.1/tcp/9933
Listening on /ip4/192.168.126.85/tcp/9933
Listening on /ip4/172.17.0.1/tcp/9933
Listening on /ip4/172.19.0.1/tcp/9933
Listening on /ip4/172.18.0.1/tcp/9933
Listening on /ip4/127.0.0.1/udp/9933/quic-v1
Listening on /ip4/192.168.126.85/udp/9933/quic-v1
Listening on /ip4/172.17.0.1/udp/9933/quic-v1
Listening on /ip4/172.19.0.1/udp/9933/quic-v1
Listening on /ip4/172.18.0.1/udp/9933/quic-v1
BehaviourEvent: Sent { connection_id: ConnectionId(1), peer_id: PeerId("12D3KooWB3mTEfi3tvZNw3kt5Q2yYb9JQyT2sKoAevdMrYti3SjF") }
BehaviourEvent: Received { connection_id: ConnectionId(1), peer_id: PeerId("12D3KooWB3mTEfi3tvZNw3kt5Q2yYb9JQyT2sKoAevdMrYti3SjF"), info: Info { public_key: PublicKey { publickey: Ed25519(PublicKey(compressed): 124a90acd492d9ee6e24cd6e41ce0acd631ef871d4549992c6aacb3e3916) }, protocol_version: "/ipfs/id/1.0.0", agent_version: "rust-libp2p/0.47.0", listen_addrs: [/ip4/172.17.0.1/tcp/41103, /ip4/127.0.0.1/tcp/41103, /ip4/172.18.0.1/tcp/41103, /ip4/192.168.126.85/tcp/41103, /ip4/172.19.0.1/tcp/41103], protocols: ["/ipfs/id/push/1.0.0", "/ipfs/id/1.0.0"], observed_addr: /ip4/127.0.0.1/tcp/9933 } }
BehaviourEvent: Event { peer: PeerId("12D3KooWB3mTEfi3tvZNw3kt5Q2yYb9JQyT2sKoAevdMrYti3SjF"), connection: ConnectionId(1), result: Err(Unsupported) }

```









# 错误

```
分析一下：

relay_server: 结果：cargo run --bin relay_server -- --secret-key-seed 42 --port 9933
   Compiling test_libp2p v0.1.0 (/home/algo/rust_space/TDdeploy/coding_tips/tutor/test_libp2p)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.52s
     Running `/home/algo/rust_space/TDdeploy/target/debug/relay_server --secret-key-seed 42 --port 9933`
Listening on /ip4/127.0.0.1/tcp/9933
Listening on /ip4/192.168.126.85/tcp/9933
Listening on /ip4/172.17.0.1/tcp/9933
Listening on /ip4/172.19.0.1/tcp/9933
Listening on /ip4/172.18.0.1/tcp/9933
Listening on /ip4/127.0.0.1/udp/9933/quic-v1
Listening on /ip4/192.168.126.85/udp/9933/quic-v1
Listening on /ip4/172.17.0.1/udp/9933/quic-v1
Listening on /ip4/172.19.0.1/udp/9933/quic-v1
Listening on /ip4/172.18.0.1/udp/9933/quic-v1
xxxzgc: BehaviourEvent: Sent { connection_id: ConnectionId(1), peer_id: PeerId("12D3KooWHqNNw24AHWgqTuWvt2VGvmtrvZdNB6J4V2YKJWGpyEwQ") }
xxxzgc: BehaviourEvent: Received { connection_id: ConnectionId(1), peer_id: PeerId("12D3KooWHqNNw24AHWgqTuWvt2VGvmtrvZdNB6J4V2YKJWGpyEwQ"), info: Info { public_key: PublicKey { publickey: Ed25519(PublicKey(compressed): 771f50566147b457668014e7c39b266c58e8394d3aa81e96a6eb5ec872bd2df7) }, protocol_version: "/ipfs/id/1.0.0", agent_version: "rust-libp2p/0.47.0", listen_addrs: [/ip4/127.0.0.1/tcp/46545, /ip4/172.19.0.1/tcp/46545, /ip4/172.17.0.1/tcp/46545, /ip4/192.168.126.85/tcp/46545, /ip4/172.18.0.1/tcp/46545], protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0"], observed_addr: /ip4/127.0.0.1/tcp/9933 } }
xxxzgc: BehaviourEvent: Event { peer: PeerId("12D3KooWHqNNw24AHWgqTuWvt2VGvmtrvZdNB6J4V2YKJWGpyEwQ"), connection: ConnectionId(1), result: Err(Unsupported) }


客户端结果：

cargo run --bin identify-example /ip4/127.0.0.1/tcp/9933
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `/home/algo/rust_space/rust-libp2p/target/debug/identify-example /ip4/127.0.0.1/tcp/9933`
Dialed /ip4/127.0.0.1/tcp/9933
Listening on /ip4/127.0.0.1/tcp/46545
Listening on /ip4/192.168.126.85/tcp/46545
Listening on /ip4/172.17.0.1/tcp/46545
Listening on /ip4/172.19.0.1/tcp/46545
Listening on /ip4/172.18.0.1/tcp/46545
Sent identify info to PeerId("12D3KooWR2KSRQWyanR1dPvnZkXt296xgf3FFn8135szya3zYYwY")
Received Info { public_key: PublicKey { publickey: Ed25519(PublicKey(compressed): e1ef2fe6f211f7399a8a6a55fdc811ee92ec7f1ee125942da87ef659553499f) }, protocol_version: "/ipfs/id/1.0.0", agent_version: "rust-libp2p/0.45.0", listen_addrs: [/ip4/192.168.126.85/udp/9933/quic-v1, /ip4/172.19.0.1/udp/9933/quic-v1, /ip4/172.17.0.1/udp/9933/quic-v1, /ip4/172.17.0.1/tcp/9933, /ip4/172.18.0.1/tcp/9933, /ip4/172.19.0.1/tcp/9933, /ip4/127.0.0.1/tcp/9933, /ip4/172.18.0.1/udp/9933/quic-v1, /ip4/192.168.126.85/tcp/9933, /ip4/127.0.0.1/udp/9933/quic-v1], protocols: ["/ipfs/ping/1.0.0", "/ipfs/id/1.0.0", "/libp2p/circuit/relay/0.2.0/hop", "/ipfs/id/push/1.0.0"], observed_addr: /ip4/127.0.0.1/tcp/40614, signed_peer_record: None }




为什么 relay_server 报错：xxxzgc: BehaviourEvent: Event { peer: PeerId("12D3KooWHqNNw24AHWgqTuWvt2VGvmtrvZdNB6J4V2YKJWGpyEwQ"), connection: ConnectionId(1), result: Err(Unsupported) }
```

这个错误 **`Err(Unsupported)`** 通常表示**协议协商失败**——即一方尝试使用某个协议，但另一方声明不支持该协议。

从你的日志来看，具体原因是：**`relay_server` 启用了 `ping` 协议，但连接的客户端（`identify-example`）没有启用 `ping`，导致 ping 探测失败。**







refer:

https://zhuanlan.zhihu.com/p/643357754





