



用于测试 kafka agsspi 的方式：

```
# 1. 克隆项目
git clone https://github.com/confluentinc/kafka-security-playground.git
cd kafka-security-playground/kerberos  # 找到 kerberos 目录

# 2. 启动整个环境（包含 Kerberos KDC + Kafka 集群）
docker-compose up -d

# 3. 等待 30-60 秒（KDC 和 Kafka 初始化）
docker-compose logs -f kafka  # 查看日志确认启动成功

# 4. 生成客户端认证文件
# 项目会自动在 ./private 目录生成 keytab 和配置文件
ls ./private  # 能看到 kafka-client.keytab, krb5.conf, *.jaas.conf
```





kimi:

```
给我使用 docker 镜像搭建一个支持 gssapi kerberos 的 kafka 集群用户测试，所有的临时输出放到当前目录下的 tmp 目录，只能使用 docker 或者 docker compose 方式搭建，不允许随便安装本地库
```













