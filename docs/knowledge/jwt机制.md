

JWT（JSON Web Token）是一种**无状态**的认证机制，核心思想是**服务器不保存会话，所有认证信息都存储在 Token 中**。

## 1. JWT 机制详解

### 结构（Header.Payload.Signature）



复制

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9  // Header: 算法+类型
.eyJ1c2VyX2lkIjoxLCJleHAiOjE3MDc1NjQ4MDB9  // Payload: 数据+过期时间
.SflKxwRJSMeKKF2QT4fwpMe...  // Signature: 防篡改签名
```

### 认证流程

1. **登录**：验证用户名密码 → 颁发 JWT（Access Token + Refresh Token）
2. **存储**：前端存储 Token（Memory/LocalStorage/Cookie）
3. **请求**：前端在 `Authorization: Bearer <token>` 头中携带
4. **验证**：后端验证签名和过期时间 → 解析用户身份
5. **刷新**：Access Token 过期时用 Refresh Token 换取新的





