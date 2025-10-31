use std::env;
use futures::{StreamExt, TryStreamExt};
use pulsar::{
    authentication::{basic::BasicAuthentication, oauth2::OAuth2Authentication}, consumer::data::MessageData, Authentication, Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor
};
use serde::{Deserialize, Serialize};

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

// const TUYAUS_URL: &str = "pulsar+ssl://mqe.tuyaus.com:7285/";
const TUYAUS_URL: &str = "pulsar://mqe.tuyaus.com:7285/";

pub struct EnvInfo {
    pub env: String,
    pub topic: String,
    pub desc: String,
}
pub enum Evn {
    PROD,
    TEST,
}
impl Evn {
    pub fn env_info(&self) -> EnvInfo {
        match self {
            Self::PROD => EnvInfo {
                env: "prod".to_string(),
                topic: "event".to_string(),
                desc: "online environment".to_string(),
            },
            Self::TEST => EnvInfo {
                env: "test".to_string(),
                topic: "event-test".to_string(),
                desc: "test environment".to_string(),
            },
        }
    }
}

// 单 topic 测试 last_message_id
pub async fn consumer_main() -> anyhow::Result<()> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| TUYAUS_URL.to_string());
        
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "49rmt4r5ukgu3rayuxcr/out/event-test".to_string());

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    let content = std::fs::read_to_string("/root/secret.key")?;
    let (username, password) = content.trim().split_once(':').unwrap();
    unsafe {
        env::set_var("PULSAR_BASIC_USERNAME", username);
        env::set_var("PULSAR_BASIC_PASSWORD", password);
    }

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
        println!("username: {:?}, password: {:?}", username, password);
        builder = builder.with_auth_provider(BasicAuthentication::new(&username, &password))
    }
    // builder.with_certificate_chain(certificate_chain);

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut consumer: Consumer<TestData, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name("test_consumer9")
        .with_subscription_type(SubType::Failover)
        .with_subscription("49rmt4r5ukgu3rayuxcr-sub")
        .build()
        .await?;

    // let s = consumer.into_stream();
    let topics = consumer.topics();
    log::info!("xxxzgc topics: {:?}", topics);
    let ids = consumer.consumer_id();
    log::info!("xxxzgc consumer_id: {:?}", ids);
    let last_msg_id = consumer.get_last_message_id().await?;
    log::info!("xxxzgc last_msg_id: {:?}", last_msg_id);

    let mut counter = 0usize;
    while let Some(msg) = consumer.try_next().await? {
    // while let Some(msg) = consumer.next().await {
        log::info!("metadata: {:?}", msg.metadata());
        log::info!("id: {:?}", msg.message_id());
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        // panic!("test no ack");
        // consumer.ack(&msg).await?;
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
