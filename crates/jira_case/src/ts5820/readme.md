# 测试脚本

//1. oracle 数据迁移到 tdengine
//2. tdengine 数据迁移到 tdengine
//3. tmq 数据迁移到 tdengine
//4. kafka 数据迁移到 tdengine
//5. tdengine 数据迁移到 local 本地文件

一些测试脚本：
some shell scirpt:
update t2 set time="2025-09-22 09:55:40" where id=7;

td2td:
taosx run -f "taos+ws://127.0.0.1:6041/mysql_st?compression=false&end=2025-09-22T18:00:00+08:00&mode=history&schema=always&schema-polling-interval=5s&sparse=false&stables=meters&start=2025-09-22T00:00:00+08:00&workers=0&write-concurrency=1" -t "taos://root:taosdata@td1:6030/ts5820" -p "@./td2td-parser.json"


```
INSERT INTO `t1` using `meters` (`groupid`,`location`) tags(1,"BJ") (`ts`,`id`,`voltage`,`v_blob`) values(now,1,11,'\x123456');
```

td2td parser:
```

```

备份是通过 tmq2local 实现的:
```
tmq+http://root:taosdata@fractal-x:6041/xaa25c7eda02?auto.offset.reset=earliest&compression=true&experimental.snapshot.enable=true&group.id=xaa25c7eda02
```


```
curl 'http://127.0.0.1:6060/api/x/backup/6/points' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: q=0.8, en' \
  -H 'Authorization: Basic cm9vdDp0YW9zZGF0YQ==' \
  -H 'Connection: keep-alive' \
  -H 'Cookie: login_TDC=true; TDengine-Token=Basic%20cm9vdDp0YW9zZGF0YQ==' \
  -H 'Referer: http://127.0.0.1:6060/management/backup' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36' \
  -H 'sec-ch-ua: "Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"'


local:/root/zgc/dev/TS-5820_blob/backup


[
    {
        "task_id": "6",
        "topic": "xaa25c7eda02",
        "db_name": "ts5820",
        "db_sql": "CREATE DATABASE `ts5820` BUFFER 256 CACHESIZE 1 CACHEMODEL 'none' COMP 2 DURATION 10d WAL_FSYNC_PERIOD 3000 MAXROWS 4096 MINROWS 100 STT_TRIGGER 2 KEEP 3650d,3650d,3650d PAGES 256 PAGESIZE 4 PRECISION 'ms' REPLICA 1 WAL_LEVEL 1 VGROUPS 2 SINGLE_STABLE 0 TABLE_PREFIX 0 TABLE_SUFFIX 0 TSDB_PAGESIZE 4 WAL_RETENTION_PERIOD 3600 WAL_RETENTION_SIZE 0 KEEP_TIME_OFFSET 0 ENCRYPT_ALGORITHM 'none' SS_CHUNKPAGES 131072 SS_KEEPLOCAL 525600m SS_COMPACT 1 COMPACT_INTERVAL 0d COMPACT_TIME_RANGE 0d,0d COMPACT_TIME_OFFSET 0h",
        "stable_name": "meters",
        "stable_sql": "CREATE STABLE `meters` (`ts` TIMESTAMP, `id` INT, `voltage` INT, `v_blob` BLOB) TAGS (`groupid` INT, `location` VARCHAR(24))",
        "point": "2025-09-25T13:40:00.359Z",
        "file_size": "472 B",
        "file_count": 2
    }
]
```

local2tmq  恢复任务：
```
curl 'http://127.0.0.1:6060/api/x/tasks' \
  -H 'Accept: application/json, text/plain, */*' \
  -H 'Accept-Language: q=0.8, en' \
  -H 'Authorization: Basic cm9vdDp0YW9zZGF0YQ==' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json;charset=UTF-8' \
  -H 'Cookie: login_TDC=true; TDengine-Token=Basic%20cm9vdDp0YW9zZGF0YQ==' \
  -H 'Origin: http://127.0.0.1:6060' \
  -H 'Referer: http://127.0.0.1:6060/management/backup' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36' \
  -H 'sec-ch-ua: "Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  --data-raw $'{"labels":["type::restore","cluster-id::1142220560582260409"],"trigger":{"schedule":"oneshot","resume":"never"},"from":"local:/root/zgc/dev/TS-5820_blob/backup?s3_enable=false&task_id=6&topic=xaa25c7eda02&from=2025-09-25T13:40:00.359Z&to=2025-09-25T13:40:00.359Z&db_name=ts5820&db_sql=CREATE DATABASE `ts5820` BUFFER 256 CACHESIZE 1 CACHEMODEL \'none\' COMP 2 DURATION 10d WAL_FSYNC_PERIOD 3000 MAXROWS 4096 MINROWS 100 STT_TRIGGER 2 KEEP 3650d,3650d,3650d PAGES 256 PAGESIZE 4 PRECISION \'ms\' REPLICA 1 WAL_LEVEL 1 VGROUPS 2 SINGLE_STABLE 0 TABLE_PREFIX 0 TABLE_SUFFIX 0 TSDB_PAGESIZE 4 WAL_RETENTION_PERIOD 3600 WAL_RETENTION_SIZE 0 KEEP_TIME_OFFSET 0 ENCRYPT_ALGORITHM \'none\' SS_CHUNKPAGES 131072 SS_KEEPLOCAL 525600m SS_COMPACT 1 COMPACT_INTERVAL 0d COMPACT_TIME_RANGE 0d,0d COMPACT_TIME_OFFSET 0h&stable_name=meters&stable_sql=CREATE STABLE `meters` (`ts` TIMESTAMP, `id` INT, `voltage` INT, `v_blob` BLOB) TAGS (`groupid` INT, `location` VARCHAR(24))","to":"tmq+http://root:taosdata@fractal-x:6041/ts5820"}'
  ```


### td2local  没有这种导出方式，只有 tmq2local
```
taosx run -f "taos+ws://127.0.0.1:6041/ts5820?query=select tbname, * from meters"  -t "local:/root/zgc/dev/TS-5820_blob/local_files"
```

### local2td
```
taosx run -f "local:/root/zgc/dev/TS-5820_blob/backup" -t "taos+ws://127.0.0.1:6041/recoverdb"
```

### local2tmq 恢复任务

### tmq2td 和 tmq2local
tmq2td 的数据不会被 tmq2local 的方式导出。只有 insert 语句会被 tmq2local 导出。
```
```

### td2csv
```
taosx run -f "taos:///testdb?query=select tbname, * from meters"   -t "csv:./meters.csv"
```

### td2parquet