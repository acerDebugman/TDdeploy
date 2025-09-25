#!/bin/bash


# td2td
#/usr/local/taos/bin/taosx run -f "taos+ws://127.0.0.1:6041/db1?compression=false&end=2025-09-22T18:00:00+08:00&mode=history&schema=always&schema-polling-interval=5s&sparse=false&stables=mysql_st&start=2025-09-22T00:00:00+08:00&workers=0&write-concurrency=1" -t "taos://root:taosdata@172.18.0.2:6030/ts5820" -p "@./td2td-parser.json"

# td2csv
/usr/local/taos/bin/taosx run -f "taos+ws://127.0.0.1:6041/db1?query=select tbname, * from meters"   -t "csv:./meters.csv"

# td2parquet
/usr/local/taos/bin/taosx run -f "taos+ws://127.0.0.1:6041/db1?query=select tbname, * from meters"   -t "parquet:./meters.parquet"
