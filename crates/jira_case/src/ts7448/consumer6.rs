#[macro_use]
extern crate serde;
use std::env;

use futures::TryStreamExt;
use pulsar::{
    authentication::{basic::BasicAuthentication, oauth2::OAuth2Authentication},
    Authentication, Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor,
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

// 测试 TLS
#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "non-persistent://public/default/test".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/abc*".to_string());
    let topic_zgc = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/zgc".to_string());

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
    builder.with_certificate_chain(certificate_chain);

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut consumer: Consumer<TestData, _> = pulsar
        .consumer()
        // .with_topic(topic)
        .with_topics(vec![topic_zgc])
        .with_topic_regex(regex::Regex::new(topic.as_str()).unwrap())
        .with_consumer_name("test_consumer3")
        // .with_subscription_type(SubType::Exclusive)
        .with_subscription_type(SubType::Shared)
        // .with_subscription_type(SubType::Failover)
        .with_subscription("test_subscription")
        // .with_subscription("test_subscription2")
        .build()
        .await?;

    let mut counter = 0usize;
    while let Some(msg) = consumer.try_next().await? {
        log::info!("metadata: {:?}", msg.metadata());
        log::info!("id: {:?}", msg.message_id());
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        // panic!("test no ack");
        consumer.ack(&msg).await?;
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                log::error!("could not deserialize message: {:?}", e);
                break;
            }
        };

        if data.data.as_str() != "data" {
            log::error!("Unexpected payload: {}", &data.data);
            break;
        }
        counter += 1;
        log::info!("got {} messages", counter);

        if counter > 100 {
            consumer.close().await.expect("Unable to close consumer");
            break;
        }
    }

    Ok(())
}
