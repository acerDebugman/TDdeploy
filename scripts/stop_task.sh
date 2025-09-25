#!/bin/bash

for n in `seq 15 51`; do
curl "http://172.20.0.2:6060/api/x/tasks/${n}/stop" \
  -X 'POST' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: zh-CN,zh;q=0.9,en;q=0.8' \
  -H 'Authorization: Basic cm9vdDp0YW9zZGF0YQ==' \
  -H 'Connection: keep-alive' \
  -H 'Content-Length: 0' \
  -b 'login_TDC=true; TDengine-Token=Basic%20cm9vdDp0YW9zZGF0YQ==' \
  -H 'Origin: http://172.20.0.2:6060' \
  -H 'Referer: http://172.20.0.2:6060/dataIn/Task' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36' \
  --insecure
done
