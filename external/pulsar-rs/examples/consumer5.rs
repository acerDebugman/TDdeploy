#[macro_use]
extern crate serde;
use std::env;

use futures::TryStreamExt;
use log::debug;
use pulsar::{
    authentication::{basic::BasicAuthentication, oauth2::OAuth2Authentication}, consumer::InitialPosition, Authentication, Consumer, ConsumerOptions, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor
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

/// 通过 REST API 删除订阅
async fn unsubscribe_http(
    topic: &str,
    sub: &str,
    pulsar_web: &str, // 如 http://localhost:8080
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}/admin/v2/persistent/{}/subscription/{}",
        pulsar_web, topic, sub
    );
    let resp = reqwest::Client::new().delete(&url).send().await?;
    if !resp.status().is_success() {
        return Err(format!("删除失败：{}", resp.text().await?).into());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let pulsar_web = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "non-persistent://public/default/test".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/abc*".to_string());
    let topic_zgc = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/zgc".to_string());
    let topic_abc = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/abc123".to_string());

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

    let pulsar: Pulsar<_> = builder.build().await?;

    let _ = unsubscribe_http(&topic_abc[13..], "test_subscription", &pulsar_web).await.unwrap();

    let mut consumer: Consumer<TestData, _> = pulsar
        .consumer()
        // .with_topic(topic)
        // .with_topic_regex(regex::Regex::new(topic.as_str()).unwrap())
        // .with_topics(vec![topic_zgc, topic_abc])
        .with_topics(vec![topic_zgc, topic])
        // .with_topics(vec![topic_zgc])
        // .with_topics(vec![topic_abc])
        .with_consumer_name("test_consumer3")
        // .with_subscription_type(SubType::Exclusive)
        .with_subscription_type(SubType::Shared)
        // .with_subscription_type(SubType::Failover)
        .with_subscription("test_subscription")
        .with_options(ConsumerOptions {
                // initial_position: InitialPosition::Latest,
                initial_position: InitialPosition::Earliest,
                ..Default::default()
            })
        // .with_subscription("test_subscription2")
        .build()
        .await?;

    let connected_topics = consumer.topics();
    debug!(
        "connected topics for {}: {:?}",
        consumer.subscription(),
        &connected_topics
    );

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
