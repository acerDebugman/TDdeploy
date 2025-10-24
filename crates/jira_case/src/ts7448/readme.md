
## 基础参数
1. subscription_name: 必须使用，需要根据 partition 来拆分任务吗？如果并发去消费？
任务数量有限制吗? share 模式下，pulsar 理论应该上应该没有限制
2. topic_name: 必须，
3. tls: 是否需要开启 tls 认证？

4. subscription 概念类似 kafka 的 consumer group
5. partitioned topic: 正常一个 topic 只能被一个 broker 支持，这可能限制吞吐； partition topic 是可以被一组 broker 支持；


## 基本使用
python 访问 pulsar 示例:
```
pip install pulsar-client
```

consumer:
```
import pulsar

client = pulsar.Client('pulsar://localhost:6650')
consumer = client.subscribe('my-topic', subscription_name='my-sub')

while True:
    msg = consumer.receive()
    print("Received message: '%s'" % msg.data())
    consumer.acknowledge(msg)

client.close()
```

producer:
```
import pulsar

client = pulsar.Client('pulsar://localhost:6650')
producer = client.create_producer('my-topic')

for i in range(10):
    producer.send(('hello-pulsar-%d' % i).encode('utf-8'))

client.close()
```

refer:
https://pulsar.apache.org/docs/4.1.x/getting-started-docker/


### 命令行使用
登录进入 broker 添加topic:
```
pulsar-admin topics create-partitioned-topic \
    persistent://public/default/pt-zgc \
    --partitions 12
```


## 测试命令

```
cargo test --package source-pulsar --lib -- config::connect::tests::test_parse_broker_url --exact --show-output --nocapture
```


## todo
### 需要添加表
information_schema.ins_grants_full 里添加 pulsar 的支持；
```
select `limits` from {} where grant_name='{connector}'

select `limits` from information_schema.ins_grants_full where grant_name='pulsar';
select `limits` from information_schema.ins_grants_full where grant_name='kafka';
```


## 问题
那么为什么 pulsar 的 java/c++ 客户端都会要求提供 ca.cert.pem 文件？为什么 pulsar-rs 里不用？
Java/C++ 客户端把 根证书选择权 暴露给用户（可自定义 CA、可开关系统根库）；
pulsar-rs 则直接把 根证书验证 交给 操作系统（或 rustls-native-certs），不再提供自定义 CA 的入口，因此代码里看不到 ca.cert.pem 参数。

4. 如果我真的要用私有 CA？
权宜办法：
把私有根证书 导入系统信任库（update-ca-trust、keychain、certutil 等）；
或者 反向代理层（Pulsar Proxy）做 TLS 终结，Rust 侧只连 Proxy 的公共 CA 端口；
等待 pulsar-rs 后续版本 暴露 root_certificates 参数。
一句话总结
Java/C++ 把 “信任谁” 交给用户，所以让你传 ca.cert.pem；
pulsar-rs 目前 强制使用操作系统根证书池，不暴露自定义 CA 入口，因此代码里看不到根证书文件——不是不需要，而是“由系统代管”。