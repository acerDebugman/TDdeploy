use chrono::Utc;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
pub async fn main() {
    //let kafka_addr = "172.19.0.6:9092"; // 替换为你的Kafka地址
    let kafka_addr = "172.19.0.3:9092"; // 替换为你的Kafka地址
    let topic = "test-topic"; // 替换为你的topic
    let duration = Duration::from_secs(3600000); // 持续10秒
    println!("topic: {}", topic);

    // kafka 发送数据
    send2kafka(kafka_addr, topic, duration).await;
}

/*
    帮我实现这个函数，函数名为send2kafka，函数的功能是将数据发送到kafka, 不用返回值。
    函数的参数为：
    1. kafka的地址
    2. kafka的topic
    3. 持续时间
    实现功能为：
    1. 连接kafka
    2. 发送数据
    3. 关闭连接
    具体功能：
    1. 每隔1秒将数据发送到kafka,直到持续时间结束
    2. 数据为：
     {
        "ts": chrono::Utc::now().timestamp_millis(),
        "id": i % 3,
        "voltage": 0.7 + i as f32,
        "groupid": i % 3,
        "location": "BeiJing"
    }
    i 是从0开始的，每隔1秒加1
*/
pub async fn send2kafka(kafka_addr: &str, kafka_topic: &str, duration: std::time::Duration) {
    // 创建Kafka生产者
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_addr)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let start_time = Instant::now();
    let mut i = 0;

    // 在持续时间内循环发送数据
    while start_time.elapsed() < duration {
        // 构造消息数据
        let message = json!({
            "ts": Utc::now().timestamp_millis(),
            "id": i % 3,
            "voltage": 0.7 + i as f32,
            "groupid": i % 3,
            "location": "BeiJing"
        })
        .to_string();

        let key = format!("key-{}", i);
        // 发送消息到Kafka
        let record = FutureRecord::to(kafka_topic).payload(&message).key(&key);

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                println!("成功发送消息: {}", message)
            }
            Err((e, _)) => eprintln!("发送消息失败: {}", e),
        }

        i += 1;
        sleep(Duration::from_millis(500)).await;
    }
}
