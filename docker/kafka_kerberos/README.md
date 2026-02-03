# Kafka + Kerberos (GSSAPI) 测试集群

## 集群状态

| 服务 | 状态 | 地址 |
|------|------|------|
| Kerberos KDC | ✅ Healthy | localhost:88 (UDP/TCP) |
| Zookeeper | ✅ Running | localhost:2181 |
| Kafka Broker | ✅ Running | localhost:9092 (SASL_PLAINTEXT) |

## Kerberos 配置

- **Realm**: EXAMPLE.COM
- **KDC**: kerberos.example.com

### Principals

| Principal | 用途 | 密码/Keytab |
|-----------|------|-------------|
| `admin/admin@EXAMPLE.COM` | Kerberos 管理员 | admin |
| `kafka/kafka.example.com@EXAMPLE.COM` | Kafka 服务主体 | kafka.keytab |
| `kafka/localhost@EXAMPLE.COM` | Kafka 本地主体 | kafka.keytab |
| `kafka-user@EXAMPLE.COM` | Kafka 测试用户 | kafka / kafka-user.keytab |
| `client@EXAMPLE.COM` | 客户端主体 | client / client.keytab |
| `zookeeper/zookeeper.example.com@EXAMPLE.COM` | Zookeeper 主体 | zookeeper.keytab |

## 快速测试

### 1. 进入客户端容器

```bash
docker exec -it kafka-client bash
```

### 2. 创建 Topic (使用 SASL/GSSAPI)

```bash
export KAFKA_OPTS="-Djava.security.krb5.conf=/etc/krb5.conf -Djava.security.auth.login.config=/etc/kafka/client-jaas.conf"

cat > /tmp/client.properties << EOF
security.protocol=SASL_PLAINTEXT
sasl.mechanism=GSSAPI
sasl.kerberos.service.name=kafka
EOF

kafka-topics.sh \
  --bootstrap-server kafka-broker:9092 \
  --command-config /tmp/client.properties \
  --create --topic test-topic \
  --partitions 1 --replication-factor 1
```

### 3. 查看 Topics

```bash
kafka-topics.sh \
  --bootstrap-server kafka-broker:9092 \
  --command-config /tmp/client.properties \
  --list
```

### 4. 生产消息

```bash
kafka-console-producer.sh \
  --bootstrap-server kafka-broker:9092 \
  --producer.config /tmp/client.properties \
  --topic test-topic
```

### 5. 消费消息

```bash
kafka-console-consumer.sh \
  --bootstrap-server kafka-broker:9092 \
  --consumer.config /tmp/client.properties \
  --topic test-topic \
  --from-beginning
```

## 文件位置

- `docker-compose.yml` - Docker Compose 配置
- `kerberos/krb5.conf` - Kerberos 客户端配置
- `kerberos/keytabs/` - Keytab 文件目录
- `kafka/kafka-jaas.conf` - Kafka JAAS 配置
- `client/client-jaas.conf` - 客户端 JAAS 配置

## 启动/停止

```bash
# 启动集群
docker compose up -d

# 停止集群
docker compose down

# 完全清理（包括 keytabs）
docker compose down -v
rm -rf kerberos/keytabs/*
```

## 注意事项

1. Kafka Broker 配置了 `SASL_PLAINTEXT` 监听器，仅支持 GSSAPI 认证
2. Zookeeper 未启用 Kerberos 认证（简化配置）
3. 所有数据存储在 Docker volumes 中，重启后会保留
4. Keytab 文件通过 volume 挂载共享给各个容器
