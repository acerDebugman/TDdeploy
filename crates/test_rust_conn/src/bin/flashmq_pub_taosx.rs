use paho_mqtt as mqtt;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> mqtt::Result<()> {
    // 1. 创建 MQTT 客户端
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")  
        .client_id("rust-producer")
        .finalize();

    let client = mqtt::Client::new(create_opts)?;

    // 2. 连接配置
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    println!("Connecting to MQTT broker...");
    let _srv_resp= client.connect(conn_opts)?;

    // 3. 发布消息
    // let topic = "rust/demo";
    let topic = "meters_mqtt";
    println!("topic: {topic}");
    
    for i in 0..10 {
        let payload = json!({
            "ts": chrono::Utc::now().timestamp_millis(),
            "id": i % 3,
            "voltage": 0.7 + i as f32,
            "groupid": i % 3,
            "location": "BeiJing"
        });
        let msg = mqtt::Message::new(
            topic,
            payload.to_string(),
            mqtt::QOS_1,
        );
        
        println!("Publishing: {}", String::from_utf8_lossy(msg.payload()));
        client.publish(msg)?;
        sleep(Duration::from_secs(1)).await;
    }

    // 4. 断开连接
    client.disconnect(None)?;
    Ok(())
}
