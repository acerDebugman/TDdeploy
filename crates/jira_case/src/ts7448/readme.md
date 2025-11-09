基础参数

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

列出所有的 topic:

```
pulsar-admin topics list public/defaulta
```

查看 订阅 cursor 位置:

```
# 持久化订阅
pulsar-admin topics stats persistent://public/default/pt-zgc-partition-1


# 非持久化订阅
bin/pulsar-admin topics stats-non-persistent non-persistent://public/default/your-topic -s your-sub
```

stats-internal 可以看到具体 subscription 的 cursor 信息:
pulsar-admin topics stats-internal persistent://public/default/pt-zgc-partition-1

```
broker:/pulsar$ pulsar-admin topics stats-internal persistent://public/default/pt-zgc-partition-1
{
  "entriesAddedCounter" : 10,
  "numberOfEntries" : 10,
  "totalSize" : 1260,
  "currentLedgerEntries" : 0,
  "currentLedgerSize" : 0,
  "lastLedgerCreatedTimestamp" : "2025-10-24T18:44:12.299Z",
  "waitingCursorsCount" : 2,
  "pendingAddEntriesCount" : 0,
  "lastConfirmedEntry" : "94:9",
  "state" : "LedgerOpened",
  "ledgers" : [ {
    "ledgerId" : 94,
    "entries" : 10,
    "size" : 1260,
    "offloaded" : false,
    "underReplicated" : false
  }, {
    "ledgerId" : 146,
    "entries" : 0,
    "size" : 0,
    "offloaded" : false,
    "underReplicated" : false
  } ],
  "cursors" : {
    "test_subscription" : {
      "markDeletePosition" : "94:9",
      "readPosition" : "94:10",
      "waitingReadOp" : true,
      "pendingReadOps" : 0,
      "messagesConsumedCounter" : 9,
      "cursorLedger" : 261,
      "cursorLedgerLastEntry" : 31,
      "individuallyDeletedMessages" : "[]",
      "lastLedgerSwitchTimestamp" : "2025-10-25T08:02:54.255Z",
      "state" : "Open",
      "active" : true,
      "numberOfEntriesSinceFirstNotAckedMessage" : 1,
      "totalNonContiguousDeletedMessagesRange" : 0,
      "subscriptionHavePendingRead" : true,
      "subscriptionHavePendingReplayRead" : false,
      "properties" : { }
    },
    "taosx-test" : {
      "markDeletePosition" : "94:3",
      "readPosition" : "94:5",
      "waitingReadOp" : false,
      "pendingReadOps" : 0,
      "messagesConsumedCounter" : 4,
      "cursorLedger" : 242,
      "cursorLedgerLastEntry" : 0,
      "individuallyDeletedMessages" : "[]",
      "lastLedgerSwitchTimestamp" : "2025-10-25T06:42:54.254Z",
      "state" : "Open",
      "active" : false,
      "numberOfEntriesSinceFirstNotAckedMessage" : 2,
      "totalNonContiguousDeletedMessagesRange" : 0,
      "subscriptionHavePendingRead" : false,
      "subscriptionHavePendingReplayRead" : false,
      "properties" : { }
    },
    "s1" : {
      "markDeletePosition" : "94:8",
      "readPosition" : "94:10",
      "waitingReadOp" : true,
      "pendingReadOps" : 0,
      "messagesConsumedCounter" : 9,
      "cursorLedger" : 259,
      "cursorLedgerLastEntry" : 0,
      "individuallyDeletedMessages" : "[]",
      "lastLedgerSwitchTimestamp" : "2025-10-25T07:50:54.254Z",
      "state" : "Open",
      "active" : true,
      "numberOfEntriesSinceFirstNotAckedMessage" : 2,
      "totalNonContiguousDeletedMessagesRange" : 0,
      "subscriptionHavePendingRead" : true,
      "subscriptionHavePendingReplayRead" : false,
      "properties" : { }
    }
  },
  "schemaLedgers" : [ {
    "ledgerId" : 17,
    "entries" : 1,
    "size" : 102,
    "offloaded" : false,
    "underReplicated" : false
  } ],
  "compactedLedger" : {
    "ledgerId" : -1,
    "entries" : -1,
    "size" : -1,
    "offloaded" : false,
    "underReplicated" : false
  }
}
```

具体执行：

```
broker:/pulsar$ pulsar-admin topics stats persistent://public/default/pt-zgc-partition-1
{
  "msgRateIn" : 0.0,
  "msgThroughputIn" : 0.0,
  "msgRateOut" : 0.0,
  "msgThroughputOut" : 0.0,
  "bytesInCounter" : 1260,
  "msgInCounter" : 10,
  "systemTopicBytesInCounter" : 0,
  "bytesOutCounter" : 8694,
  "msgOutCounter" : 69,
  "bytesOutInternalCounter" : 0,
  "averageMsgSize" : 0.0,
  "msgChunkPublished" : false,
  "storageSize" : 1260,
  "backlogSize" : 756,
  "backlogQuotaLimitSize" : -1,
  "backlogQuotaLimitTime" : -1,
  "oldestBacklogMessageAgeSeconds" : 51983,
  "oldestBacklogMessageSubscriptionName" : "taosx-test",
  "publishRateLimitedTimes" : 0,
  "earliestMsgPublishTimeInBacklogs" : 0,
  "offloadedStorageSize" : 0,
  "lastOffloadLedgerId" : 0,
  "lastOffloadSuccessTimeStamp" : 0,
  "lastOffloadFailureTimeStamp" : 0,
  "ongoingTxnCount" : 0,
  "abortedTxnCount" : 0,
  "committedTxnCount" : 0,
  "publishers" : [ ],
  "waitingPublishers" : 0,
  "subscriptions" : {
    "test_subscription" : {
      "msgRateOut" : 0.0,
      "msgThroughputOut" : 0.0,
      "bytesOutCounter" : 7560,
      "msgOutCounter" : 60,
      "msgRateRedeliver" : 0.0,
      "messageAckRate" : 0.0,
      "chunkedMessageRate" : 0.0,
      "msgBacklog" : 1,
      "backlogSize" : 0,
      "earliestMsgPublishTimeInBacklog" : 0,
      "msgBacklogNoDelayed" : 1,
      "blockedSubscriptionOnUnackedMsgs" : false,
      "msgDelayed" : 0,
      "msgInReplay" : 0,
      "unackedMessages" : 0,
      "type" : "Failover",
      "activeConsumerName" : "test_consumer9",
      "msgRateExpired" : 0.0,
      "msgExpired" : 0,
      "totalMsgExpired" : 0,
      "lastExpireTimestamp" : 0,
      "lastConsumedFlowTimestamp" : 1761383019060,
      "lastConsumedTimestamp" : 1761383019961,
      "lastAckedTimestamp" : 1761383046982,
      "lastMarkDeleteAdvancedTimestamp" : 1761383046983,
      "consumers" : [ {
        "msgRateOut" : 0.0,
        "msgThroughputOut" : 0.0,
        "bytesOutCounter" : 1260,
        "msgOutCounter" : 10,
        "msgRateRedeliver" : 0.0,
        "messageAckRate" : 0.0,
        "chunkedMessageRate" : 0.0,
        "consumerName" : "test_consumer9",
        "availablePermits" : 990,
        "unackedMessages" : 0,
        "avgMessagesPerEntry" : 1,
        "blockedConsumerOnUnackedMsgs" : false,
        "drainingHashesCount" : 0,
        "drainingHashesClearedTotal" : 0,
        "drainingHashesUnackedMessages" : 0,
        "address" : "/172.21.0.1:36280",
        "connectedSince" : "2025-10-25T09:03:39.018756859Z",
        "clientVersion" : "pulsar-rs-v6.4.1",
        "lastAckedTimestamp" : 1761383046982,
        "lastConsumedTimestamp" : 1761383019961,
        "firstMessagesSentTimestamp" : 1761383019962,
        "lastConsumedFlowTimestamp" : 1761383019060,
        "firstConsumedFlowTimestamp" : 1761383019060,
        "metadata" : { },
        "lastAckedTime" : "2025-10-25T09:04:06.982Z",
        "lastConsumedTime" : "2025-10-25T09:03:39.961Z"
      } ],
      "isDurable" : true,
      "isReplicated" : false,
      "allowOutOfOrderDelivery" : false,
      "consumersAfterMarkDeletePosition" : { },
      "drainingHashesCount" : 0,
      "drainingHashesClearedTotal" : 0,
      "drainingHashesUnackedMessages" : 0,
      "nonContiguousDeletedMessagesRanges" : 0,
      "nonContiguousDeletedMessagesRangesSerializedSize" : 0,
      "delayedMessageIndexSizeInBytes" : 0,
      "subscriptionProperties" : { },
      "filterProcessedMsgCount" : 0,
      "filterAcceptedMsgCount" : 0,
      "filterRejectedMsgCount" : 0,
      "filterRescheduledMsgCount" : 0,
      "dispatchThrottledMsgEventsBySubscriptionLimit" : 0,
      "dispatchThrottledBytesEventsBySubscriptionLimit" : 0,
      "dispatchThrottledMsgEventsByTopicLimit" : 0,
      "dispatchThrottledBytesEventsByTopicLimit" : 0,
      "dispatchThrottledMsgEventsByBrokerLimit" : 0,
      "dispatchThrottledBytesEventsByBrokerLimit" : 0,
      "durable" : true,
      "replicated" : false
    },
    "taosx-test" : {
      "msgRateOut" : 0.0,
      "msgThroughputOut" : 0.0,
      "bytesOutCounter" : 504,
      "msgOutCounter" : 4,
      "msgRateRedeliver" : 0.0,
      "messageAckRate" : 0.0,
      "chunkedMessageRate" : 0.0,
      "msgBacklog" : 6,
      "backlogSize" : 756,
      "earliestMsgPublishTimeInBacklog" : 0,
      "msgBacklogNoDelayed" : 6,
      "blockedSubscriptionOnUnackedMsgs" : false,
      "msgDelayed" : 0,
      "msgInReplay" : 0,
      "unackedMessages" : 0,
      "type" : "Failover",
      "msgRateExpired" : 0.0,
      "msgExpired" : 0,
      "totalMsgExpired" : 0,
      "lastExpireTimestamp" : 0,
      "lastConsumedFlowTimestamp" : 1761320601544,
      "lastConsumedTimestamp" : 0,
      "lastAckedTimestamp" : 0,
      "lastMarkDeleteAdvancedTimestamp" : 0,
      "consumers" : [ ],
      "isDurable" : true,
      "isReplicated" : false,
      "allowOutOfOrderDelivery" : false,
      "consumersAfterMarkDeletePosition" : { },
      "drainingHashesCount" : 0,
      "drainingHashesClearedTotal" : 0,
      "drainingHashesUnackedMessages" : 0,
      "nonContiguousDeletedMessagesRanges" : 0,
      "nonContiguousDeletedMessagesRangesSerializedSize" : 0,
      "delayedMessageIndexSizeInBytes" : 0,
      "subscriptionProperties" : { },
      "filterProcessedMsgCount" : 0,
      "filterAcceptedMsgCount" : 0,
      "filterRejectedMsgCount" : 0,
      "filterRescheduledMsgCount" : 0,
      "dispatchThrottledMsgEventsBySubscriptionLimit" : 0,
      "dispatchThrottledBytesEventsBySubscriptionLimit" : 0,
      "dispatchThrottledMsgEventsByTopicLimit" : 0,
      "dispatchThrottledBytesEventsByTopicLimit" : 0,
      "dispatchThrottledMsgEventsByBrokerLimit" : 0,
      "dispatchThrottledBytesEventsByBrokerLimit" : 0,
      "durable" : true,
      "replicated" : false
    },
    "s1" : {
      "msgRateOut" : 0.0,
      "msgThroughputOut" : 0.0,
      "bytesOutCounter" : 630,
      "msgOutCounter" : 5,
      "msgRateRedeliver" : 0.0,
      "messageAckRate" : 0.0,
      "chunkedMessageRate" : 0.0,
      "msgBacklog" : 1,
      "backlogSize" : 126,
      "earliestMsgPublishTimeInBacklog" : 0,
      "msgBacklogNoDelayed" : 1,
      "blockedSubscriptionOnUnackedMsgs" : false,
      "msgDelayed" : 0,
      "msgInReplay" : 0,
      "unackedMessages" : 0,
      "type" : "Failover",
      "activeConsumerName" : "c1",
      "msgRateExpired" : 0.0,
      "msgExpired" : 0,
      "totalMsgExpired" : 0,
      "lastExpireTimestamp" : 0,
      "lastConsumedFlowTimestamp" : 1761320710883,
      "lastConsumedTimestamp" : 1761321567596,
      "lastAckedTimestamp" : 1761321537399,
      "lastMarkDeleteAdvancedTimestamp" : 1761321537401,
      "consumers" : [ {
        "msgRateOut" : 0.0,
        "msgThroughputOut" : 0.0,
        "bytesOutCounter" : 630,
        "msgOutCounter" : 5,
        "msgRateRedeliver" : 0.0,
        "messageAckRate" : 0.0,
        "chunkedMessageRate" : 0.0,
        "consumerName" : "c1",
        "availablePermits" : 995,
        "unackedMessages" : 0,
        "avgMessagesPerEntry" : 1,
        "blockedConsumerOnUnackedMsgs" : false,
        "drainingHashesCount" : 0,
        "drainingHashesClearedTotal" : 0,
        "drainingHashesUnackedMessages" : 0,
        "address" : "/172.21.0.1:40310",
        "connectedSince" : "2025-10-24T15:45:10.882816886Z",
        "clientVersion" : "pulsar-rs-v6.4.1",
        "lastAckedTimestamp" : 1761321537399,
        "lastConsumedTimestamp" : 1761321567596,
        "firstMessagesSentTimestamp" : 1761320764551,
        "lastConsumedFlowTimestamp" : 1761320710883,
        "firstConsumedFlowTimestamp" : 1761320710883,
        "metadata" : { },
        "lastAckedTime" : "2025-10-24T15:58:57.399Z",
        "lastConsumedTime" : "2025-10-24T15:59:27.596Z"
      } ],
      "isDurable" : true,
      "isReplicated" : false,
      "allowOutOfOrderDelivery" : false,
      "consumersAfterMarkDeletePosition" : { },
      "drainingHashesCount" : 0,
      "drainingHashesClearedTotal" : 0,
      "drainingHashesUnackedMessages" : 0,
      "nonContiguousDeletedMessagesRanges" : 0,
      "nonContiguousDeletedMessagesRangesSerializedSize" : 0,
      "delayedMessageIndexSizeInBytes" : 0,
      "subscriptionProperties" : { },
      "filterProcessedMsgCount" : 0,
      "filterAcceptedMsgCount" : 0,
      "filterRejectedMsgCount" : 0,
      "filterRescheduledMsgCount" : 0,
      "dispatchThrottledMsgEventsBySubscriptionLimit" : 0,
      "dispatchThrottledBytesEventsBySubscriptionLimit" : 0,
      "dispatchThrottledMsgEventsByTopicLimit" : 0,
      "dispatchThrottledBytesEventsByTopicLimit" : 0,
      "dispatchThrottledMsgEventsByBrokerLimit" : 0,
      "dispatchThrottledBytesEventsByBrokerLimit" : 0,
      "durable" : true,
      "replicated" : false
    }
  },
  "replication" : { },
  "deduplicationStatus" : "Disabled",
  "nonContiguousDeletedMessagesRanges" : 0,
  "nonContiguousDeletedMessagesRangesSerializedSize" : 0,
  "delayedMessageIndexSizeInBytes" : 0,
  "compaction" : {
    "lastCompactionRemovedEventCount" : 0,
    "lastCompactionSucceedTimestamp" : 0,
    "lastCompactionFailedTimestamp" : 0,
    "lastCompactionDurationTimeInMills" : 0
  },
  "ownerBroker" : "broker:8080",
  "topicCreationTimeStamp" : 1761280295889,
  "lastPublishTimeStamp" : 1761321567588
}


```

代码：

```
let latest_id_data = MessageIdData {
        ledger_id: u64::MAX,
        entry_id: u64::MAX,
        ..Default::default()
    };
    // let latest_id_data  = last_msg_id;
    consumer.seek(Some(consumer.topics()), Some(latest_id_data.clone()), None, pulsar).await?;
    log::info!("seek to latest_id_data: {:?}", latest_id_data);
```

对应的cursor:

```
"cursors" : {
    "test_subscription" : {
      "markDeletePosition" : "94:-1",
      "readPosition" : "94:0",
      "waitingReadOp" : false,
      "pendingReadOps" : 0,
      "messagesConsumedCounter" : -1,
      "cursorLedger" : 261,
      "cursorLedgerLastEntry" : 36,
      "individuallyDeletedMessages" : "[]",
      "lastLedgerSwitchTimestamp" : "2025-10-25T08:02:54.255Z",
      "state" : "Open",
      "active" : false,
      "numberOfEntriesSinceFirstNotAckedMessage" : 1,
      "totalNonContiguousDeletedMessagesRange" : 0,
      "subscriptionHavePendingRead" : false,
      "subscriptionHavePendingReplayRead" : false,
      "properties" : { }
    },
```

图中的 lastConfirmedEntry 应该是代码的返回值：

```
let last_msg_id = consumer.get_last_message_id().await?;
```

因为测试发现，不论 scription name: test_subscription 的 makrDeletePosition 和 readPosition 怎么变化，都不影响 get_last_message_id() 的返回值信息。

即使换另外一个订阅名： test_subscription2， 函数：get_last_message_id() 返回的依旧是 94:9 的位置。

图：

![alt text](image.png)

## 测试命令

```
cargo test --package source-pulsar --lib -- config::connect::tests::test_parse_broker_url --exact --show-output --nocapture

```

### 命令行执行 pulsar 导入 ：

```
taosx run -f "pulsar://192.168.2.131:6650?batch_size=1000&busy_threshold=100%&char_encoding=UTF_8&consumer_name=c1&initial_position=Earliest&subscription=zgc&timeout=0ms&topics=persistent://public/default/pt-zgc" -t "taos+http://root:taosdata@192.168.2.131:6041/zgc" -p "@./docs/taosx/pulsar-parser.json"


```

## 依赖组件

```
apt-get install protobuf-compiler
```

## 代码逻辑：

1. taos-ui/componnets/views/sourceConfig.vue  才是 数据源配置 的页面。
2. 所有数据源的页面，应该是根据每个 json文件的 配置，动态渲染产生的 单独页面。所以应该是改内部的判断字段。

transform 可以实现的功能：这种复杂的 json, 可以通过 parser json 解析第一层，然后再

```
{
  "data": {
    "dataId": "000642FBF5506604B3863BC268080499",
    "status": [
      {
        "2": "2300",
        "code": "temp_current",
        "t": 1762499780177,
        "value": 2300
      },
      {
        "2": "444",
        "code": "tttt",
        "t": 1762499780177,
        "value": 777
      }
    ]
  },
  "protocol": 4,
  "pv": "2.0",
  "sign": "b86c4d071e2172477f4b4826d773e45e",
  "t": 1762499780403
}
```

## todo

### 需要添加表

information_schema.ins_grants_full 里添加 pulsar 的支持；

```
select `limits` from {} where grant_name='{connector}'

select `limits` from information_schema.ins_grants_full where grant_name='pulsar';
select `limits` from information_schema.ins_grants_full where grant_name='kafka';
```

### 现有todo

20251025

1. 少量数据滞留问题, 少量数据会滞留，不会发送出去，现象：一条数据无法 发送出去, batch 发送的 timeout 问题 （done）
2. breakpoint 如何做？(done)
   1. 目前依赖 pulsar 的 cursor, 不手动做了；
3. metrics 如何做？(done)
   1. 添加新的 metrics (done)
4. 后续是否需要需要改为 shared 模式？(xx暂时放弃)
5. 测试用例 (done)
6. jwt_token, basic_auth, mtls 测试 (xx暂时放弃)
7. 任务列表页面 list 上，似乎不可以选择任务 (done)
   1. 可以选择，开了开发工具导致 (done)
8. 命令行 (done)
9. agent 测试 （done）
10. 性能测试
11. 涂鸦的 文档查看, 发现内部有加密工具，可能需要进行 二次 解密的开发 (done)
12. broker 多地址确认，只支持单地址 (done)
13. 删除掉 xxxzgc 注释 **
14. broker_url 点击编辑进入后 broker_url 为空 **
15. 创建 transform 多个 key 的问题 **
16. explorer/.env 文件恢复 (done)
17. agent 模式下的 do_put 流里也需要支持 pulsar, 但是为什么 测试 没报错?
18. 增加 pulsar-tuya 数据源 (done)
19. 错误改造 decryptor snafu (done)
    1. 也不见得比 thiserror 好用，下次不用了
20. ins_grants_full 支持 **
21. 补充部分 consumer 的测试用例
22. 涂鸦命令行测试 **
23. metrics 有新数据进来会自动停止 **

pulsar-rs bug:

1. exclude 模式下，seek 会死循环
2. seek 使用 u64::MAX 不能推到最新的 data record, 那么用 seek，就几乎只能 seek 历史的信息了

其他：

1. 确认 pular-rs 还有其他的方式获取当前 topic 的最新的 ledger_id 和 entry_id 吗
2. pulsar-rs 研究下代码和设计

## 开启认证

broker 的 broker.conf

```

# --- 启用 Basic 认证 ---
authenticationEnabled=true
authenticationProviders=org.apache.pulsar.broker.authentication.AuthenticationProviderBasic
basicAuthConf=file:///pulsar/basic-auth/.htpasswd

# --- Broker 之间互调用也走 Basic ---
brokerClientAuthenticationPlugin=org.apache.pulsar.client.impl.auth.AuthenticationBasic
brokerClientAuthenticationParameters={"userId":"root","password":"taosdata"}

# --- 如果后面加 Proxy，同样配 ---
forwardAuthorizationCredentials=true
```

```

mkdir basic-auth
apt-get install -y apache2-utils
# 新建用户 superuser/admin
htpasswd -cB ./basic-auth/.htpasswd root
# 再追加一个客户端账号
htpasswd -B ./basic-auth/.htpasswd taosdata

```

将 broker.conf 拷贝出来修改：

```
 docker cp broker:/pulsar/conf/broker.conf .
```

docker-compose.yaml 里的配置：

```
  # Start broker
  broker:
    image: apachepulsar/pulsar:latest
    container_name: broker
    hostname: broker
    restart: on-failure
    networks:
      - pulsar
    environment:
      - metadataStoreUrl=zk:zookeeper:2181
      - zookeeperServers=zookeeper:2181
      - clusterName=cluster-a
      - managedLedgerDefaultEnsembleSize=1
      - managedLedgerDefaultWriteQuorum=1
      - managedLedgerDefaultAckQuorum=1
      - advertisedAddress=broker
      - advertisedListeners=external:pulsar://127.0.0.1:6650
      - PULSAR_MEM=-Xms512m -Xmx512m -XX:MaxDirectMemorySize=256m
    volumes:
      - /root/zgc/dev/dev_setup/TDdeploy/crates/jira_case/src/ts7448/basic-auth:/pulsar/basic-auth
      - /root/zgc/dev/dev_setup/TDdeploy/crates/jira_case/src/ts7448/basic-auth/broker.conf:/pulsar/conf/broker.conf
    depends_on:
      zookeeper:
        condition: service_healthy
      bookie:
        condition: service_started
    ports:
      - "6650:6650"
      - "8080:8080"
    command: bash -c "bin/apply-config-from-env.py conf/broker.conf && exec bin/pulsar broker"

```

测试：

vi conf/client.conf

```
authPlugin=org.apache.pulsar.client.impl.auth.AuthenticationBasic
authParams={"userId":"superuser","password":"admin"}
```

```
pulsar-admin --admin-url http://localhost:8080   tenants list
```

## 测试数据

```
{"ts":1761320604889,"id":0,"v_str":"255044462D312E330D0A","groupid":0,"location":"BeiJing"} {"ts":1761320605896,"id":1,"v_str":"255044462D312E330D0A","groupid":1,"location":"BeiJing"} {"ts":1761320606911,"id":2,"v_str":"255044462D312E330D0A","groupid":2,"location":"BeiJing"} {"ts":1761320607924,"id":0,"v_str":"255044462D312E330D0A","groupid":0,"location":"BeiJing"} {"ts":1761320608935,"id":1,"v_str":"255044462D312E330D0A","groupid":1,"location":"BeiJing"}
```

#### 涂鸦客户端使用

进入  sdk 位置, 编写测试用例：

```
cd /home/algo/rust_space/tuya-pulsar-sdk-java

# 非 pom.xml 修改的打包的执行方法：
mvn clean package
mvn dependency:copy-dependencies -DoutputDirectory=target/lib

java -cp "open-mq-sdk/target/open-mq-sdk-1.0-SNAPSHOT.jar:open-mq-sdk/target/lib/*"  com.tuya.open.sdk.example.ConsumerExample

```

消息样例：

```

2025-10-31 06:46:04.564, INFO, com.tuya.open.sdk.example.ConsumerExample, ConsumerExample.java, main, 27, lambda$main$0 ###TUYA_PULSAR_MSG => start process message, messageId=195828067:1:-1, publishTime=1761893118723, encryptModel=aes_gcm, payload={"data":"OTNi0JBHstQL9PKzcIrWZwHilrvcOK2h79oS+by+jMbQ2LcvA65CH5LtTBt+8EpLgMerQdCWL6aSTx/DGujxxiVb0PjFzJG6SvCszC9Fm3F3hqQh+2SbHC6DnzbCRrijzITlETBZtmGcHt+RyRDpSQVYd2J8iyjcvQsz5NT7YyDVXG7pHWzWuW3goC6huzYZUPxGDxllssHwTk26OWWzuLshxzs1X9onBGMubk7NFZbpiP+reZNjreBHeLC+sjhCwsisw4ley1jYsWsL32ac/cf+8XTRa7w=","encryptVersion":"v2","protocol":4,"pv":"2.0","sign":"1e235eac0b37b6a8dc59ad7de9caf878","t":1761893118723}


###TUYA_PULSAR_MSG => decrypt messageVO={"data":"OTNi0JBHstQL9PKzcIrWZwHilrvcOK2h79oS+by+jMbQ2LcvA65CH5LtTBt+8EpLgMerQdCWL6aSTx/DGujxxiVb0PjFzJG6SvCszC9Fm3F3hqQh+2SbHC6DnzbCRrijzITlETBZtmGcHt+RyRDpSQVYd2J8iyjcvQsz5NT7YyDVXG7pHWzWuW3goC6huzYZUPxGDxllssHwTk26OWWzuLshxzs1X9onBGMubk7NFZbpiP+reZNjreBHeLC+sjhCwsisw4ley1jYsWsL32ac/cf+8XTRa7w=","protocol":4,"pv":"2.0","sign":"1e235eac0b37b6a8dc59ad7de9caf878","t":1761893118723}
data after decryption dataJsonStr={"dataId":"0006426EB5731E8B7DA5A0BF68071196","devId":"ebc778f3c5d9908ff6plgl","productKey":"9exm2qiar0dvqoxv","status":[{"3":"52","code":"humidity_current","t":1761893118322,"value":52}]} messageId=195828067:1:-1


2025-10-31 06:46:04.634, INFO, com.tuya.open.sdk.example.ConsumerExample, ConsumerExample.java, main, 30, lambda$main$0 ###TUYA_PULSAR_MSG => finish process message, messageId=195828067:1:-1, publishTime=1761893118723, encryptModel=aes_gcm


```

key位置： /root/secret.key

加密的 comman data:

```
xxxzgc: command data: {"username":"49rmt4r5ukgu3rayuxcr","password":"ac591f4f689ba2f9"}
xxxzgc: command data: {"username":"49rmt4r5ukgu3rayuxcr","password":"ac591f4f689ba2f9"}
xxxzgc: command data: {"username":"49rmt4r5ukgu3rayuxcr","password":"ac591f4f689ba2f9"}

```

pulsar-rs 的消息：

```

xxxzgc*** auth_data: Some(Authentication { name: "49rmt4r5ukgu3rayuxcr", data: [123, 34, 112, 97, 115, 115, 119, 111, 114, 100, 34, 58, 34, 97, 99, 53, 57, 49, 102, 52, 102, 54, 56, 57, 98, 97, 50, 102, 57, 34, 44, 34, 117, 115, 101, 114, 110, 97, 109, 101, 34, 58, 34, 52, 57, 114, 109, 116, 52, 114, 53, 117, 107, 103, 117, 51, 114, 97, 121, 117, 120, 99, 114, 34, 125] }), data_str: {"password":"ac591f4f689ba2f9","username":"49rmt4r5ukgu3rayuxcr"}

```

使用 pulsar-rs 鉴权上，还是有问题，和

#### 登录测试机器

使用 .pem 文件登录服务器

```
(base) algo@algo-PC:~/tmp/tuya$ chmod 400 tuya-test_key.pem 
(base) algo@algo-PC:~/tmp/tuya$ ssh taosdata@52.249.217.13 -i tuya-test_key.pem

文件传输:
scp -i tuya-test_key.pem amazon-corretto-8-x64-linux-jdk.tar.gz  taosdata@52.249.217.13:~
```

### 一些测试用例代码

```

#[cfg(test)]
mod tests {
    use taos::IntoDsn;


    #[tokio::test]
    #[ignore]
    async fn test_pulsar_to_taos() {
        let dsn = format!(
            "pulsar://{}?ca={}&ca_password=abcdefgh&cert={}&cert_key={}",
            "192.168.2.131:6650",
            "@../tests/kafka/ca-cert",
            "@../tests/kafka/client_test_client.pem",
            "@../tests/kafka/client_test_client.key",
        )
        .into_dsn()
        .expect("ssl dsn should be valid");

    }

}

```

数据样例：从样例数据发现：

1. data 里的数据格式不一样
2. 甚至 status 里的数据格式也不一样

可以说明：无法用统一的 transform 导入数据，只能直接存储 json string

```
taos> select last(*) from tuya \G;
*************************** 1.row ***************************
  last(ts): 2025-11-09 01:21:09.901
last(data): {"bizCode":"deviceOnline","bizData":{"devId":"eb04108544a7478d53rvtr","uid":"az1756942079386rVhsK","productId":"c8idttdahj9y6eaw","time":1762651269796},"ts":1762651269898}
last(sign): 8169e9432c65bf3f4cdfc4bbca97ca62
   last(t): 1762651269901
Query OK, 1 row(s) in set (0.002240s)

taos> select last(*) from tuya \G;
*************************** 1.row ***************************
  last(ts): 2025-11-09 01:21:19.949
last(data): {"dataId":"0006431F3B62131FBC782DBF68140803","devId":"ebc778f3c5d9908ff6plgl","productKey":"9exm2qiar0dvqoxv","status":[{"3":"44","code":"humidity_current","t":1762651279595,"value":44}]}
last(sign): 7d714d25925bb4f50c4fc29af6ed2ddb
   last(t): 1762651279949
Query OK, 1 row(s) in set (0.002219s)

taos> select last(*) from tuya \G;
*************************** 1.row ***************************
  last(ts): 2025-11-09 01:21:19.949
last(data): {"dataId":"0006431F3B62131FBC782DBF68140803","devId":"ebc778f3c5d9908ff6plgl","productKey":"9exm2qiar0dvqoxv","status":[{"3":"44","code":"humidity_current","t":1762651279595,"value":44}]}
last(sign): 7d714d25925bb4f50c4fc29af6ed2ddb
   last(t): 1762651279949
Query OK, 1 row(s) in set (0.002219s)

taos> select last(*) from tuya \G;
*************************** 1.row ***************************
  last(ts): 2025-11-09 01:22:22.188
last(data): {"dataId":"0006431F3F176B32C3E096C2680902EB","devId":"eb9ce1146ab84d9872spvt","productKey":"plwbuwzx","status":[{"1":"-55","code":"va_temperature","t":1762651341812,"value":-55}]}
last(sign): 1ff377a9112b2b9c8d1f3f552669f03b
   last(t): 1762651342188
Query OK, 1 row(s) in set (0.002056s)


```

### 创建任务请求命令行

```

curl 'http://52.249.217.13:6060/api/x/tasks' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: zh-CN,zh;q=0.9' \
  -H 'Authorization: Basic cm9vdDp0YW9zZGF0YQ==' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json' \
  -H 'Cookie: login_TDC=true; TDengine-Token=Basic%20cm9vdDp0YW9zZGF0YQ==' \
  -H 'Origin: http://52.249.217.13:6060' \
  -H 'Referer: http://52.249.217.13:6060/dataIn/add' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.6533.100 Safari/537.36' \
  --data-raw '{"from":"","from_json":{"agent":"","type":"pulsarTuya","data":{"endpoint":"mqe.tuyaus.com:7285","tuya_access_id":"49rmt4r5ukgu3rayuxcr","tuya_access_key":"fbe6805862cc4527a90e782967c79b31","tuya_env":"test","timeout":"0ms","initial_position":"Earliest","char_encoding":"UTF_8","read_concurrency":0,"batch_size":1000,"written_concurrent":null,"health_check_window_in_second":"0s","busy_threshold":"100%","max_queue_length":1000,"max_errors_in_window":10}},"name":"tuya1","to":"taos+http://root:taosdata@tuya-test:6041/testdb","labels":["type::datain","cluster-id::1980594558856407762","user::root"],"parser":{"parser":{"global":{"cache":{"max_size":"1GB","location":"","on_fail":"skip"},"archive":{"keep_days":"30d","max_size":"1GB","location":"","on_fail":"rotate"},"database_connection_error":"cache","database_not_exist":"break","table_not_exist":"retry","primary_timestamp_overflow":"archive","primary_timestamp_null":"archive","primary_key_null":"archive","table_name_length_overflow":"archive","table_name_contains_illegal_char":{"replace_to":""},"variable_not_exist_in_table_name_template":{"replace_to":""},"field_name_not_found":"add_field","field_name_length_overflow":"archive","field_length_extend":true,"field_length_overflow":"archive","ingesting_error":"archive","connection_timeout_in_second":"30s"},"parse":{"value":{"json":""}},"model":{"name":"t_${protocol}","using":"tuya","tags":["protocol","pv"],"columns":["ts","data","sign","t"]},"mutate":[{"map":{"ts":{"cast":"t","as":"TIMESTAMP(ms)"},"data":{"cast":"data","as":"VARCHAR"},"sign":{"cast":"sign","as":"VARCHAR"},"t":{"cast":"t","as":"BIGINT"},"protocol":{"cast":"protocol","as":"BIGINT"},"pv":{"cast":"pv","as":"VARCHAR"}}}]},"input":[{"value":"{\"data\":\"{\\\"dataId\\\":\\\"0006431D37586434BC782DBF68140721\\\",\\\"devId\\\":\\\"ebc778f3c5d9908ff6plgl\\\",\\\"productKey\\\":\\\"9exm2qiar0dvqoxv\\\",\\\"status\\\":[{\\\"3\\\":\\\"45\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642621916,\\\"value\\\":45}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"bf206ca28c260c92379148d50dab3c6a\",\"t\":1762642622109}","key":"ebc778f3c5d9908ff6plgl"},{"value":"{\"data\":\"{\\\"dataId\\\":\\\"0006431D38939B49C3E096C2680902C1\\\",\\\"devId\\\":\\\"eb9ce1146ab84d9872spvt\\\",\\\"productKey\\\":\\\"plwbuwzx\\\",\\\"status\\\":[{\\\"1\\\":\\\"-175\\\",\\\"code\\\":\\\"va_temperature\\\",\\\"t\\\":1762642642576,\\\"value\\\":-175}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"fbe1153e520807517d0e1a1562135bff\",\"t\":1762642642881}","key":"eb9ce1146ab84d9872spvt"},{"value":"{\"data\":\"{\\\"dataId\\\":\\\"0006431D3895018FC3E096C2680902C2\\\",\\\"devId\\\":\\\"eb9ce1146ab84d9872spvt\\\",\\\"productKey\\\":\\\"plwbuwzx\\\",\\\"status\\\":[{\\\"2\\\":\\\"970\\\",\\\"code\\\":\\\"va_humidity\\\",\\\"t\\\":1762642642667,\\\"value\\\":970}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"62dd8af0091c52ca419760b04d13ac79\",\"t\":1762642642999}","key":"eb9ce1146ab84d9872spvt"},{"value":"{\"data\":\"{\\\"dataId\\\":\\\"0006431D3E76EC57BC782DBF68140725\\\",\\\"devId\\\":\\\"ebc778f3c5d9908ff6plgl\\\",\\\"productKey\\\":\\\"9exm2qiar0dvqoxv\\\",\\\"status\\\":[{\\\"3\\\":\\\"43\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642741358,\\\"value\\\":43}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"db99f70677200d93c6c2f5a05ae1b761\",\"t\":1762642741722}","key":"ebc778f3c5d9908ff6plgl"},{"value":"{\"data\":\"{\\\"dataId\\\":\\\"0006431D4824CDEF76A1CCBF680F1D49\\\",\\\"devId\\\":\\\"eba9e60b9b453cc668tjns\\\",\\\"productKey\\\":\\\"ef7aaqdzdaqggar7\\\",\\\"status\\\":[{\\\"3\\\":\\\"52\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642903750,\\\"value\\\":52}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"d11e55eea0e60b0e71158b8d9ef974a2\",\"t\":1762642903916}","key":"eba9e60b9b453cc668tjns"}],"format":{"pageCount":7,"pageSize":20,"currentPage":1}}}' \
  --insecure
```

parser sqlite db 里的：

```

{
  "parser": {
    "global": {
      "cache": {
        "max_size": "1GB",
        "location": "",
        "on_fail": "skip"
      },
      "archive": {
        "keep_days": "30d",
        "max_size": "1GB",
        "location": "",
        "on_fail": "rotate"
      },
      "database_connection_error": "cache",
      "database_not_exist": "break",
      "table_not_exist": "retry",
      "primary_timestamp_overflow": "archive",
      "primary_timestamp_null": "archive",
      "primary_key_null": "archive",
      "table_name_length_overflow": "archive",
      "table_name_contains_illegal_char": {
        "replace_to": ""
      },
      "variable_not_exist_in_table_name_template": {
        "replace_to": ""
      },
      "field_name_not_found": "add_field",
      "field_name_length_overflow": "archive",
      "field_length_extend": true,
      "field_length_overflow": "archive",
      "ingesting_error": "archive",
      "connection_timeout_in_second": "30s"
    },
    "parse": {
      "value": {
        "json": ""
      }
    },
    "model": {
      "name": "t_${protocol}",
      "using": "tuya",
      "tags": [
        "protocol",
        "pv"
      ],
      "columns": [
        "ts",
        "data",
        "sign",
        "t"
      ]
    },
    "mutate": [
      {
        "map": {
          "ts": {
            "cast": "t",
            "as": "TIMESTAMP(ms)"
          },
          "data": {
            "cast": "data",
            "as": "VARCHAR"
          },
          "sign": {
            "cast": "sign",
            "as": "VARCHAR"
          },
          "t": {
            "cast": "t",
            "as": "BIGINT"
          },
          "protocol": {
            "cast": "protocol",
            "as": "BIGINT"
          },
          "pv": {
            "cast": "pv",
            "as": "VARCHAR"
          }
        }
      }
    ]
  },
  "input": [
    {
      "value": "{\"data\":\"{\\\"dataId\\\":\\\"0006431D37586434BC782DBF68140721\\\",\\\"devId\\\":\\\"ebc778f3c5d9908ff6plgl\\\",\\\"productKey\\\":\\\"9exm2qiar0dvqoxv\\\",\\\"status\\\":[{\\\"3\\\":\\\"45\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642621916,\\\"value\\\":45}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"bf206ca28c260c92379148d50dab3c6a\",\"t\":1762642622109}",
      "key": "ebc778f3c5d9908ff6plgl"
    },
    {
      "value": "{\"data\":\"{\\\"dataId\\\":\\\"0006431D38939B49C3E096C2680902C1\\\",\\\"devId\\\":\\\"eb9ce1146ab84d9872spvt\\\",\\\"productKey\\\":\\\"plwbuwzx\\\",\\\"status\\\":[{\\\"1\\\":\\\"-175\\\",\\\"code\\\":\\\"va_temperature\\\",\\\"t\\\":1762642642576,\\\"value\\\":-175}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"fbe1153e520807517d0e1a1562135bff\",\"t\":1762642642881}",
      "key": "eb9ce1146ab84d9872spvt"
    },
    {
      "value": "{\"data\":\"{\\\"dataId\\\":\\\"0006431D3895018FC3E096C2680902C2\\\",\\\"devId\\\":\\\"eb9ce1146ab84d9872spvt\\\",\\\"productKey\\\":\\\"plwbuwzx\\\",\\\"status\\\":[{\\\"2\\\":\\\"970\\\",\\\"code\\\":\\\"va_humidity\\\",\\\"t\\\":1762642642667,\\\"value\\\":970}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"62dd8af0091c52ca419760b04d13ac79\",\"t\":1762642642999}",
      "key": "eb9ce1146ab84d9872spvt"
    },
    {
      "value": "{\"data\":\"{\\\"dataId\\\":\\\"0006431D3E76EC57BC782DBF68140725\\\",\\\"devId\\\":\\\"ebc778f3c5d9908ff6plgl\\\",\\\"productKey\\\":\\\"9exm2qiar0dvqoxv\\\",\\\"status\\\":[{\\\"3\\\":\\\"43\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642741358,\\\"value\\\":43}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"db99f70677200d93c6c2f5a05ae1b761\",\"t\":1762642741722}",
      "key": "ebc778f3c5d9908ff6plgl"
    },
    {
      "value": "{\"data\":\"{\\\"dataId\\\":\\\"0006431D4824CDEF76A1CCBF680F1D49\\\",\\\"devId\\\":\\\"eba9e60b9b453cc668tjns\\\",\\\"productKey\\\":\\\"ef7aaqdzdaqggar7\\\",\\\"status\\\":[{\\\"3\\\":\\\"52\\\",\\\"code\\\":\\\"humidity_current\\\",\\\"t\\\":1762642903750,\\\"value\\\":52}]}\",\"protocol\":4,\"pv\":\"2.0\",\"sign\":\"d11e55eea0e60b0e71158b8d9ef974a2\",\"t\":1762642903916}",
      "key": "eba9e60b9b453cc668tjns"
    }
  ],
  "format": {
    "pageCount": 7,
    "pageSize": 20,
    "currentPage": 1
  }
}
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
