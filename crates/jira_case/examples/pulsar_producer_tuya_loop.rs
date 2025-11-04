#[macro_use]
extern crate serde;
use std::{env, time::{SystemTime, UNIX_EPOCH}};

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
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    // let topic = env::var("PULSAR_TOPIC")
    //     .ok()
    //     .unwrap_or_else(|| "non-persistent://public/default/test".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/tuya".to_string());


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
        .with_name("loop_producer")
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

        // let item = format!("{}", "255044462D312E330D0A");
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        // {"data":"rip/39XhBv/ib+JHwFwIh4r8GrPFii2qejc0a3xG8KriXhKrWz+8wgBLnAvHEB3yLK9AEwyShIJGsZzekNXwnct24XLX+ml7fDOWGXgQwkpuKAaMuOKPO49bV2ry7tQmbB/+WzefuKC4u2tFxqulM6qaOEklRwQzl92n5d6HTgFZXPVK4uKniSzCmH0XakAiaQ7krIn19pfDbRkp0nnkDcgFB/t/uPYSRj1/L0wr5tFyRXWVM9LlrSnW4FLx59d2HyrUqYd8j+a2Yxlb7VYrfMYctQ6eois=","encryptVersion":"v2","protocol":4,"pv":"2.0","sign":"eee8d1dec8b008c8b71ab37635365aab","t":1762221539653}
        let message = serde_json::json!({
            "data": "rip/39XhBv/ib+JHwFwIh4r8GrPFii2qejc0a3xG8KriXhKrWz+8wgBLnAvHEB3yLK9AEwyShIJGsZzekNXwnct24XLX+ml7fDOWGXgQwkpuKAaMuOKPO49bV2ry7tQmbB/+WzefuKC4u2tFxqulM6qaOEklRwQzl92n5d6HTgFZXPVK4uKniSzCmH0XakAiaQ7krIn19pfDbRkp0nnkDcgFB/t/uPYSRj1/L0wr5tFyRXWVM9LlrSnW4FLx59d2HyrUqYd8j+a2Yxlb7VYrfMYctQ6eois=",
            "encryptVersion": "v2",
            "protocol": 4,
            "pv": "2.0",
            "sign": "eee8d1dec8b008c8b71ab37635365aab",
            "t": ts,
            "encryptModel": "aes_gcm",
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

        if counter >= 1000000000 {
            producer.close().await.expect("Unable to close connection");
            break;
        }
    }
    Ok(())
}
