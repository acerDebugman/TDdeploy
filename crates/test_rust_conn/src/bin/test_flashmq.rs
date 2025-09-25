use paho_mqtt as mqtt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let broker_addr = "tcp://localhost:1883";
    let client_id = "rust_writer";
    let topic = "device/data";

    // 创建 MQTT 客户端
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(broker_addr)
        .client_id(client_id)
        .finalize();

    let client = mqtt::AsyncClient::new(create_opts).unwrap();

    // 连接到 MQTT 代理
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    
    let conn_token = client.connect(conn_opts);
    // 等待连接完成
    conn_token.wait()?;

    // 构造并发送消息
    let payload = b"Hello FlashMQ!";
    let msg = mqtt::Message::new(topic, payload.to_vec(), 1); // QoS 1
    let delivery_token = client.publish(msg);

    // 等待消息发送完成
    delivery_token.wait().unwrap();

    Ok(())
}