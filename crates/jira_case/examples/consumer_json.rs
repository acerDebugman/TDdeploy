#[macro_use]
extern crate serde;
use std::env;

use futures::{StreamExt, TryStreamExt};
use pulsar::{
    authentication::{basic::BasicAuthentication, oauth2::OAuth2Authentication}, consumer::data::MessageData, proto::MessageIdData, Authentication, Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor
};

#[derive(Serialize, Deserialize)]
struct TestData {
    data: String,
}

impl DeserializeMessage for TestData {
    type Output = Result<TestData, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

// 单 topic 测试 last_message_id
#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://192.168.2.131:6650".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "non-persistent://public/default/test".to_string());
    // 对于 non-partitioned topic，直接指定 topic 即可，使用逻辑 topic
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/pt-zgc".to_string());
    // 对于 partitioned topic，需要指定具体的 partition; 等价于用 topic 来代表 12 个 partition 了，也是合理的设计
    // 这样，consumer 就可以消费 pt-zgc 这个 topic 下的所有消息了
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/pt-zgc-partition-1".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "persistent://public/default/pt-zgc-partition-[0-2]".to_string());
    // let topic_zgc = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "persistent://public/default/zgc".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    if let Ok(token) = env::var("PULSAR_TOKEN") {
        let authentication = Authentication {
            name: "token".to_string(),
            data: token.into_bytes(),
        };

        builder = builder.with_auth(authentication);
    } else if let Ok(oauth2_cfg) = env::var("PULSAR_OAUTH2") {
        builder = builder.with_auth_provider(OAuth2Authentication::client_credentials(
            serde_json::from_str(oauth2_cfg.as_str())
                .unwrap_or_else(|_| panic!("invalid oauth2 config [{}]", oauth2_cfg.as_str())),
        ));
    } else if let (Ok(username), Ok(password)) = (
        env::var("PULSAR_BASIC_USERNAME"),
        env::var("PULSAR_BASIC_PASSWORD"),
    ) {
        builder = builder.with_auth_provider(BasicAuthentication::new(&username, &password))
    }
    // builder.with_certificate_chain(certificate_chain);

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut consumer: Consumer<Vec<u8>, _> = pulsar
        .consumer()
        // .with_topic(topic)
        // .with_topics(vec![topic_zgc])
        .with_topics(vec![topic])
        // .with_topic_regex(regex::Regex::new(topic.as_str()).unwrap())
        .with_consumer_name("test_consumer9")
        .with_subscription_type(SubType::Exclusive)
        // .with_subscription_type(SubType::Shared)
        // .with_subscription_type(SubType::Failover)
        .with_subscription("test_subscription")
        // .with_subscription("test_subscription2")
        .build()
        .await?;


    // let s = consumer.into_stream();
    let topics = consumer.topics();
    log::info!("xxxzgc topics: {:?}", topics);
    let ids = consumer.consumer_id();
    log::info!("xxxzgc consumer_id: {:?}", ids);
    let last_msg_id = consumer.get_last_message_id().await?;
    log::info!("xxxzgc last_msg_id: {:?}", last_msg_id);

    // let earliest_id_data = MessageIdData {
    //     ledger_id: u64::MAX,
    //     entry_id: u64::MAX,
    //     ..Default::default()
    // };
    // consumer.seek(Some(consumer.topics()), Some(earliest_id_data.clone()), None, pulsar).await?;
    // log::info!("seek to earliest_id_data: {:?}", earliest_id_data);
    // let latest_id_data = MessageIdData {
    //     ledger_id: u64::MAX,
    //     entry_id: u64::MAX,
    //     ..Default::default()
    // };
    // consumer.seek(Some(consumer.topics()), Some(latest_id_data), None, pulsar).await?;
    // log::info!("seek to latest_id_data: {:?}", latest_id_data);
    let msg_id_data = MessageIdData {
        ledger_id: 5,
        entry_id: 2,
        ..Default::default()
    };
    consumer.seek(Some(consumer.topics()), Some(msg_id_data.clone()), None, pulsar).await?;
    log::info!("seek to msg_id_data: {:?}", msg_id_data);

    let mut counter = 0usize;
    while let Some(msg) = consumer.try_next().await? {
    // while let Some(msg) = consumer.next().await {
        log::info!("metadata: {:?}", msg.metadata());
        log::info!("id: {:?}", msg.message_id());
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        // panic!("test no ack");
        // consumer.ack(&msg).await?;
        consumer.ack(&msg).await?;
        let data = msg.deserialize();

        let rs_json = String::from_utf8_lossy(&data);
        counter += 1;
        log::info!("got {} messages: {:?}", counter, rs_json);

        if counter > 100 {
            consumer.close().await.expect("Unable to close consumer");
            break;
        }
    }

    Ok(())
}
