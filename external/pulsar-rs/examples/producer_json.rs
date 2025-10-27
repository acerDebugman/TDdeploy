#[macro_use]
extern crate serde;
use std::env;

use pulsar::{
    authentication::{basic::BasicAuthentication, oauth2::OAuth2Authentication},
    message::proto,
    producer, Authentication, Error as PulsarError, Pulsar, SerializeMessage, TokioExecutor,
};

#[derive(Serialize, Deserialize)]
struct TestData {
    data: String,
}

impl SerializeMessage for TestData {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://192.168.2.131:6650".to_string());
        //.unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "non-persistent://public/default/test".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/pt-zgc-partition-1".to_string());
        //.unwrap_or_else(|| "persistent://public/default/pt-zgc".to_string());


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
    let mut producer = pulsar
        .producer()
        .with_topic(topic)
        .with_name("bytes_producer")
        // .with_options(producer::ProducerOptions {
        //     schema: Some(proto::Schema {
        //         // r#type: proto::schema::Type::String as i32,
        //         r#type: proto::schema::Type::Binary as i32,
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // })
        .build()
        .await?;

    let mut counter = 0usize;
    loop {

        let item = format!("{}", "255044462D312E330D0A");
        let message = serde_json::json!({
            "ts": chrono::Utc::now().timestamp_millis(),
            "id": counter % 3,
            // "voltage": 0.7 + i as f32,
            // "v_blob": item.as_bytes(),
            "v_str": item,
            "groupid": counter % 3,
            "location": "BeiJing"
        })
        .to_string();

        producer
            .send_non_blocking(message.as_bytes())
            .await?
            .await
            .unwrap();

        counter += 1;
        log::info!("{counter} messages");
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

        if counter >= 1 {
            producer.close().await.expect("Unable to close connection");
            break;
        }
    }
    Ok(())
}

