use paho_mqtt as mqtt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> mqtt::Result<()> {
    // 1. 创建 MQTT 客户端
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .client_id("rust-consumer")
        .finalize();

    let client = mqtt::Client::new(create_opts)?;

    // 2. 连接配置
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    println!("Connecting to MQTT broker...");
    client.connect(conn_opts)?;

    // 3. 订阅主题并处理消息
    // let (tx, mut rx) = mpsc::channel(1024);
    let (tx, mut rx) = mpsc::channel(1024);
    let topic_filter = "rust/demo";

    let q_rx = client.start_consuming();

    println!("Subscribing to {}", topic_filter);
    client.subscribe(topic_filter, mqtt::QOS_1)?;

    // 4. 启动消息监听循环
    let mut tasks = vec![];
    tasks.push(tokio::spawn(async move {
        while let Ok(msg) = q_rx.recv(){
            // println!("Received: {:?}", msg);
            tx.send(msg.map_or("this is default msg".to_string(), |x|String::from_utf8_lossy(x.payload()).to_string())).await.unwrap();
        }
        println!("Receiver channel closed.");
    }));

    while let Some(msg) = rx.recv().await {
        println!("Received: {:?}", msg); 
    }
    
    // tasks.push(tokio::spawn(async move {
    //     let msg = rx.recv().await;
    //     println!("Received: {:?}", msg);
    //     let _ = std::io::stdout().flush();
    //     println!("Receiver begin work.");
    //     while let Some(msg) = rx.recv().await {
    //         println!("Received: {:?}", msg); 
    //         let _ = std::io::stdout().flush();
    //     }
    //     println!("Receiver channel closed.");
    // }));

    for task in tasks {
        task.await.unwrap();
    }
    // 保持运行
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+C");
    client.disconnect(None)?;

    Ok(())
}