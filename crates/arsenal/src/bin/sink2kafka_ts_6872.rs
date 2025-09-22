use base64::engine::general_purpose;
use base64::Engine;
use rayon::vec;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{message, ClientConfig};
use chrono::{Local, Utc};
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::sleep;


#[tokio::main]
pub async fn main() {
    let kafka_addr = "172.18.0.3:9092"; // 替换为你的Kafka地址
    let topic = "test-topic"; // 替换为你的topic
    let duration = Duration::from_secs(3600000); // 持续10秒
    println!("topic: {}", topic);

    send2kafka(kafka_addr, topic, duration).await;
}


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
        // let message = json!({
        //     "ts": Utc::now().timestamp_millis(),
        //     "id": i % 3,
        //     "voltage": 0.7 + i as f32,
        //     "groupid": i % 3,
        //     "location": "BeiJing"
        // }).to_string();
        /*
        {
            "dataType": "DATA_CVI_OBD",
            "dataTime": "2025-06-27 10:49:48",
            "saveTime": "2025-06-27 10:55:34",
            "vin": "YS2G6X237M5622467",
            "payload": {
                "detectProtocol": 2,
                "milState": 2,
                "detectState": 2,
                "detectReadyState": 2,
                "identifyCode": "LBZWANGXUD0NG0826",
                "idVersion": "200000000000000002",
                "calibrateVerify": "300000000000000003",
                "IUPR": "000000000000000000000000000000000000",
                "errorCodeCount": 3,
                "errorCodes": ["0118002A", "0118002A", "0118002A"]
            }
        }
         */

        let dt = Local::now().format("%Y-%m-%d %H:%M:%S");
        let save_dt = Local::now().format("%Y-%m-%d %H:%M:%S");
        let ts = Utc::now().timestamp_millis();

        let mut message1 = json!(
            {
                "ts": Utc::now().timestamp_millis(),
                "dataType": "DATA_CVI_OBD",
                "dataTime": dt.to_string(),
                "saveTime": save_dt.to_string(),
                "vin": "YS2G6X237M5622467",
                "payload": {
                    "detectProtocol": 2,
                    "milState": i,
                    "detectState": 2,
                    "detectReadyState": 2,
                    "identifyCode": "LBZWANGXUD0NG0826",
                    "idVersion": "200000000000000002",
                    "calibrateVerify": "300000000000000003",
                    "IUPR": "0000000000000000000000000000000",
                    "errorCodeCount": i,
                    "errorCodes": ["0118002A", "0118002A", "0118002A"]
                }
            }
        ).to_string();

        // let message = r#"{"dataType":"DATA_CVI_OBD","dataTime":"2025-07-17 15:13:07","saveTime":"2025-07-17 15:13:13","vin":"L47R4X236S8000028","payload":{"detectProtocol":1,"milState":0,"detectState":52970,"detectReadyState":0,"identifyCode":"L47R4X236S8000028","idVersion":"333238373636353030303030303030303030","calibrateVerify":"203E1A1100006C3F558A7C320B24D5DD3030","IUPR":"000A0093000000000009000A00000000000900020009000A0006000A000A000A00000000","errorCodeCount":3,"errorCodes":["170E1F28","85020928","F1070928"]}}"#.to_string();
        let message = json!(
            {
                "dataType": "DATA_CVI_OBD",
                "dataTime": "2025-07-17 15:13:07",
                "saveTime": dt.to_string(),
                "vin": "L47R4X236S8000028",
                "payload": {
                    "detectProtocol": 1,
                    "milState": 0,
                    "detectState": 52970,
                    "detectReadyState": 0,
                    "identifyCode": "L47R4X236S8000028",
                    "idVersion": "333238373636353030303030303030303030",
                    "calibrateVerify": "203E1A1100006C3F558A",
                    "IUPR": "000A0093000000000009000A0000000000",
                    "errorCodeCount": 3,
                    "errorCodes": ["170E1F28", "85020928", "F1070928"]
                }
            }
        ).to_string();

        let message = if ts % 2 == 0 {
            message.as_bytes().to_vec()
        } else {
            message1.as_bytes().to_vec()
        };
        // println!("non-utf8 str: {:?}", String::from_utf8_lossy(&message));
        // let message = gbk_bytes.to_vec();
        
        // message.insert_str(18, gbk_bytes);

        // ok, 但是无法复现问题, 因为这已经是正常的utf8 了，凡是能够在页面上正常显示的都是正常的utf8
        // let message = r#"eyJkYXRhVHlwZSI6IkRBVEFfQ1ZJX09CRCIsImRhdGFUaW1lIjoiMjA1Ny0wNS0zMCAxOTowNjowNSIsInNhdmVUaW1lIjoiMjAyNS0wNy0xNCAxNzoxMDozMSIsInZpbiI6IllTMlA4WDQzOUsyMTcwMDEyIiwicGF5bG9hZCI6eyJkZXRlY3RQcm90b2NvbCI6MTgwLCJtaWxTdGF0ZSI6MTkwLCJkZXRlY3RTdGF0ZSI6MzY0NSwiZGV0ZWN0UmVhZHlTdGF0ZSI6MTc5NzIsImlkZW50aWZ5Q29kZSI6IihM77+9JlUndy3vv73vv73vv70277+9dO+/vTfvv70iLCJpZFZlcnNpb24iOiJcZu+/vSQh77+9Uyfvv70iLCJjYWxpYnJhdGVWZXJpZnkiOiJcYt2QfyIsIklVUFIiOiLvv71/IiwiZXJyb3JDb2RlQ291bnQiOjMyLCJlcnJvckNvZGVzIjpbIjE0MDMwMDIwIiwiMTQwMzAwNDAiLCIwMDAwMDAzNCIsIjAwMDAwMDgyIiwiNTcwNDAwMDAiLCIwMDAxMDFGRiIsIkZGMDAwMDM0IiwiMDAwMDAwMUMiLCIwMDA3MDAwMCIsIjAwMDEwMDAwIiwiMDAwMDAwMDAiLCIwMDAwMDAwMCIsIkJBMDBFQzhGIiwiN0YwMDAwQzAiLCJGMjAwRUM4RiIsIjdGMDAwMDAwIiwiMDAwMDAwOEYiLCI3RjAwMDA5MCIsIjAwMDAwMDAwIiwiMDAwMDAwODUiLCIwMDAwMDAwMCIsIjAwMDAwMDcwIiwiNjBCRUNDOTAiLCI3RjAwMDBBMCIsIkM1MDBFQzhGIiwiN0YwMDAwQTAiLCJDNTAwRUM4RiIsIjdGMDAwMDAwIiwiMDAwMDAwMDAiLCIwMDAwMDAwMCIsIjAwMDAwMDAwIiwiMDAwMDAwNDAiXX19"#;
        // let message = general_purpose::STANDARD.decode(message).unwrap();

        // try send empty message
        // let message = vec![0u8, 0];
        // let message = "".to_string();

        let key = format!("key-{}", i);
        // 发送消息到Kafka
        let record = FutureRecord::to(kafka_topic)
            .payload(&message)
            .key(&key);

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                println!("成功发送消息: {:?}", String::from_utf8_lossy(&message))
            },
            Err((e, _)) => eprintln!("发送消息失败: {}", e),
        }

        i += 1;
        sleep(Duration::from_millis(1000)).await;
    }

}
