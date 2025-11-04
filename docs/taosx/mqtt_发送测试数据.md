


使用命令向 mqtt 发送测试数据：

```
mosquitto_pub -h localhost -p 1883 -t test/topic -m "Hello MQTT"
```


使用命令发送：

```
cargo run -p test_rust_conn --bin flashmq_pub_taosx
```
