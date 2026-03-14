



**关键点**：即使你在连接字符串中设置 `encryption=Off`，SQL Server 2008 的 TDS 协议实现仍要求**至少加密登录包**（包含用户名密码）。因此，只要服务器返回的不是 `ENCRYPT_NOT_SUP`，就会触发 TLS 握手，进而因 TLS 版本不兼容而失败 。



| 返回值            | 值   | 含义                           |
| ----------------- | ---- | ------------------------------ |
| `ENCRYPT_OFF`     | 0x00 | 加密可用但关闭（仅登录包加密） |
| `ENCRYPT_ON`      | 0x01 | 加密可用且开启（全程加密）     |
| `ENCRYPT_NOT_SUP` | 0x02 | 加密不可用                     |
| `ENCRYPT_REQ`     | 0x03 | 强制要求加密                   |



**默认行为取决于服务器配置**：

- 如果 SQL Server 2008 **没有**在配置管理器中开启"Force Encryption"，当客户端发送 `ENCRYPT_OFF` 时，服务器会返回 `ENCRYPT_OFF`（仅加密登录包，后续明文传输）
- 如果服务器开启了"Force Encryption"，则会返回 `ENCRYPT_REQ`，强制要求全程 TLS 加密 





