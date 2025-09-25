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


 
td2td parser:
```
```