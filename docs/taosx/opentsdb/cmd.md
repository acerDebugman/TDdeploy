## 数据模型：

opentsdb 只有单列，有 tags，只有一个 列，时间戳； 没有多列。 influxdb 支持多列！

单列，ts, tags ， metrics 指标名； 就这些信息，指标名 更像是表。





### **核心差异详解**

#### **1. 数据模型：单值 vs 多值**

- **OpenTSDB**：每行数据**只能有一个 value**

  JSON

  复制

  ```json
  {"metric":"cpu","value":99.0}  // 正确
  ```

  

- **InfluxDB**：一行数据可包含**多个 field**（多值模型）

  JSON

  复制

  ```json
  {"measurement":"cpu","fields":{"idle":99.0,"user":1.0,"system":0.0}}  // 支持
  ```





## 配置

http://localhost:4242/



docker compse 配置：

```
  opentsdb:
    image: petergrace/opentsdb-docker:latest
    container_name: opentsdb
    hostname: otsdb-host
    ports:
      - "4242:4242"
      - "60030:60030"
    extra_hosts:
      - "buildkitsandbox:127.0.0.1"
    volumes:
      - /home/algo/rust_space:/app
      - ./data:/data/hbase
    environment:
      - WAITSECS=30
    networks:
      taos_net:
        ipv4_address: 172.30.0.12
    restart: unless-stopped
```



## 测试数据导入

数据导入：

```
# 进入 OpenTSDB 容器
docker exec -it opentsdb bash

# 导入 CSV 文件（格式：metric timestamp value tags...）
./bin/tsdb import -d mydb -f /path/to/data.csv
```



```
cpu.idle 1356998400 99.0 host=server1
cpu.idle 1356998401 98.0 host=server1
```



```
tsdb import -d mydb -f data.csv
```





python 脚本导入：

```
import requests
import json
import time

url = "http://localhost:4242/api/put"
data = [
    {
        "metric": "cpu.idle",
        "timestamp": int(time.time()),
        "value": 99.0,
        "tags": {"host": "server1"}
    }
]

response = requests.post(url, data=json.dumps(data))
# 成功返回 204
print(response.status_code)
```









