#!/bin/bash

for n in `seq 31 40`; do
curl 'http://172.20.0.2:6060/api/x/tasks' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: zh-CN,zh;q=0.9,en;q=0.8' \
  -H 'Authorization: Basic cm9vdDp0YW9zZGF0YQ==' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json' \
  -b 'login_TDC=true; TDengine-Token=Basic%20cm9vdDp0YW9zZGF0YQ==' \
  -H 'Origin: http://172.20.0.2:6060' \
  -H 'Referer: http://172.20.0.2:6060/dataIn/add' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36' \
  --data-raw '{"from":"","from_json":{"agent":"","type":"influxdb","data":{"protocol":"http","host":"172.20.0.3","port":"8086","version":"1.7","orgId":"","token":"","addDbrp":false,"username":"root","password":"taosdata","only-choose-one$":"1~x","bucket":"mydb","measurements":"type_614d3a6cb5289800500e81f2_1j76Rb20VWg","beginTime":"2025-08-04T00:00:00+08:00","endTime":"2025-08-05T00:00:00+08:00","readWindow":60,"delay":10,"log_level":"info","read_concurrency":50,"write_concurrency":50,"batch_size":5000,"batch_timeout":1000,"health_check_window_in_second":"0s","busy_threshold":"100%","max_queue_length":1000,"max_errors_in_window":10}},"name":"influxdb'_${n}'","to":"taos+http://root:taosdata@buildkitsandbox:6041/testdb","labels":["type::datain","cluster-id::7552860327625784333","user::root"]}' \
  --insecure
done

