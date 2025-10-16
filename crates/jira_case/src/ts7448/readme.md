
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


