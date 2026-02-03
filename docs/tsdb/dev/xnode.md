## 待讨论

1. 部分指令需要区分 用户用 还是 xnoded 用

## 开发：



### todo:

1. mqtt 模块代码理解 (done)
2. xnode 启动，目前现在初始化 open 的时候启动，后续改为 mnode 切换的时候启动，(先最小实现, 熟悉后再改) (done)
3. SXnode 结构体与 dnode 的总结构体关联上 (done)
4. 理解 消息的回调 机制？怎么回调的？(done)
   1. 再写文章理解
5. 测试 mnode 切换 (done）
   1. 3.4.0.0 已测试,  docker 环境里可以正常切换！

6. create xnode 发送消息给 xnoded, 试一试 http 消息， 在 mndCreateXnode() 组装消息发送给 xnoded (done)
7. 追一下 sql parser 模块，比如 TDMT_MND_CREATE_XNODE 怎么被发出来 (done)
8. xmnd 到 xnoded 的消息接口要用 http 协议重写一下，这样方便复用现有的 curl 接口等 (done)
9. 补全调用 create xnode tasks, show xnode tasks 功能 (done)
10. 补全调用 create xnode jobs, show xnode jobs 功能 (done)
11. create xnode task 的时候， via 默认值应该是 -1  (done)
12. dnode 关闭也需要杀死 xnoded ,需要关联 dnoded 关闭流程 (done)
13. 内存泄露测试：检查代码，看哪里还有内存泄露；压力测试 sql， 看内存是否会上涨！ (done)
    1. pytest 里的 python 框架，已经是 sql 重复执行的压力测试工具了
14. alter 修改 xnode, xnode task, xnode job 的 status 等指令，都需要发送对应的 接口 给 xnoded (done)
15. 删除 task 前需要先发送 stop task,然后删除关联 job (done)
16. status 应该是可读的：stopped, running, failed, succeeded (done)
17. create xnode 首次执行必须带 xnoded 的用户名密码，其他不用； (done)
18. 用户名密码需要保存，因为 xnoded 重启后就没有了; 用户名密码 开发：存储用户名密码 (done)
19. xnoded 启动测试，管理，首次启动：create xnode (done)
20. xnoded 随 mnode 启动 (done)
21. 代码内存清理 (done)
22. 参数校验接口 (done)
23. rebalance 自动接口，主要是 where clause 条件 (done)
24. force 参数： drop xnode force (done)
25. force 参数： drop xnode task force id (done)
26. sql 节点名称打印，for rebalance (done)
27. 用户密码第二次设置是否更新？添加一个更新逻辑
28. core dump 问题修复 (done)
29. 测试用例, 自动化写到　python 文件中  (done)
    1. 使用 pytest 进行测试，可以使用 ai 帮忙产生对应的执行 sql
30. unix socket 开发测试 (done)
31. rebalance where clause (done)
32. 删除 xnoded 测试项目 (done)
33. 删除不存在节点的测试 (done)
34. where支持：遍历所有的检查支持的 nodeType (done)
35. 编写c的测试用例: mndXnode.c 的内部函数 (done)
36. rebalance for task 功能 (done xxx不做)
37. parAstCreater 前做 nodetype 和 expression type 的检查:  SValueNode 只支持 UBigInt 和 binary 类型 (done xxx不做)
38. contain 函数支持
39. reason = NULL 的判断：literal 是 null,  且 UserAlias  以 \'  或者  \"  开头，就是字符串，否则就是 null 值， 用户输入的 NULL 会转为小写的 null， 改  valueNode 的 isNull 字段为 true; (done)
40. operator 右侧必须是 valueNode 预处理  (done xxx思考后不需要)
41. create_time 时间 where 支持对比 (done)
42. 测试 kafka 性能 (done xxx不做)
43. taosx xnode 管理指令相关官网文档 (done)
44. rebalance xnode jobs; 全量支持 (done)
45. drain 模式节点没有设置为 drain (done)
46. alter task 支持 name (done)
47. agent 管理开发支持 (done)
48. 测试文档中的测试用例 测试 (done)
49. 每次带上用户名和密码，都应该重启 xnoded (done xxx没必要的思考)
50. task, agent 修改名字需要判断名字是否已经存在，创建时候已经判断了，修改需要加此逻辑 (done)
51. 增加 agent 的 cpp 测试用例 (done)
52. 获取 mnode leader 的函数可能不对，需要再看看是不是应该使用 SDB_MNODE 这个键值！(done)
    1. 经过思考：所有的消息回调处理都在 mnode  的 leader 上执行，所以获取当前的 dnode id 就是 mnode 的 leader ID
53. 使用 token 接入代替用户名密码：token 是用户先创建，还是 mnode 里自行创建？当然是自行创建，并且应该有 root 权限的 token


#### 20260112

1. 动态 secret 产生发送给 xnoded 
2. task 的 labels 支持 (done)
3. 日志 info, debug 传递给 xnoded (done)
4. 多路径查找 xnoded 执行文件 (done)
5. 如果 xnoded panic ，会引起 taosd 不断重启 xnoded, 并且看不到日志，因为这个日志是输出到 stdout 里的！所以 systemctl 的方式看不到这种日志

#### 20260203
1.  tdinternal 长度问题开发, 目前最长只能到 59392







### corner case:

1. column 不存在， null 值， > NULL 值这样的条件







### 学习用例:

1. mndXnode 里的 evaluateWalker 的 SHashObj 的 SValueNode 只支持 UBIGINT 和 BINARY 类型; binary 就是 char 类型

2. 测试 libuv 和 调用集成
   1. 测试 c 与  rust 使用 libuv 通信, 使用 http api 接口 （done）

3. 使用 libuv 进行任务管理，发送消息等 (done)

4. 测试 bnode，试一试 mqtt (低优先级)



当前 bnode 是通过：

1. libuv 管理
2. bnode 的功能是创建一个 mqtt 协议的服务，使 tsdb 的 topic 支持 mqtt, 其他的 mqtt 客户端可以直接连接 tsdb 的 mqtt 地址，直接获取数据工作
3. libuv 是一个类似 tokio 的异步io 库，但是没有 tokio 的抽象好

目录：

```
include/libs/txnode/txnode.h   // 用于开放头文件
source/dnode/mgmt/mgmt_xnode/CMakeLists.txt  // 启动的时候使用, 包装为主函数
source/dnode/mgmt/mgmt_xnode/inc/xndInt.h
source/dnode/mgmt/mgmt_xnode/src/xnode.c
source/dnode/xnode/src/xnode.c               // 启动函数等模块
source/libs/txnode/inc/txnodeInt.h           // 具体与 xnoded 通信模块 txnode 内容
source/libs/txnode/mgmt/CMakeLists.txt
source/libs/txnode/mgmt/src/txnodeMgmt.c      // txnode 管理的
source/libs/txnode/xnode/src/txnodeDaemon.c    // 启动的任务的，libuv 管理的话，应该是启动的线程
```

整体流程：

dmMain() -> dmInit() -> dmInitDnode() ->



#### 20260112

1. 梳理执行计划，sql rewrite 框架 (done)



## blog

1. 开发流程
2. 



## 测试

1. 创建 bnode
   ```
   create bnode on dnode 1 protocol 'mqtt';
   ```

环境安装

```
apt install clang clangd lldb cmake
```

### 测试用例

```
apt install python3.10-venv

cd /app/TDengine/test

python3 -m venv .venv

# 激活
source .venv/bin/activate

# 使用uv
# pip3 install -r requirements.txt 

pip3 install uv
uv pip install -r requirements.txt 
```

执行 python 测试用例：

```
cd TDengine/test
pytest cases/42-Xnode/test_xnode.py -q

```

要只运行 `test_show_primitives` 这个测试方法，使用 pytest 的**路径::类::方法**语法：

```
# 精确匹配（推荐）
pytest cases/42-Xnode/test_xnode.py::TestXnode::test_show_primitives -v

# 或按关键字匹配（更灵活）
pytest -k "test_show_primitives" -v
```

```


LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 pytest cases/42-Xnode/test_xnode.py -q

LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest cases/42-Xnode/test_xnode.py -q

pytest cases/42-Xnode/test_xnode.py::TestXnode::test_sources_and_sinks_variants -v


LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/17-DataSubscription/02-Consume/test_tmq_vnode_split_dup_no_wal.py -N 3 --replica 3
```

### sql命令

创建 task

```
create xnode task 't1' from 'mqtt://xxx' to database s1 with parser '{...}';


# 实际数据
create xnode task 't1' from 'pulsar://192.168.2.131:6650?agent=1&batch_size=1000&busy_threshold=100%&char_encoding=UTF_8&consumer_name=c1&group=1&health_check_window_in_second=0s&initial_position=Earliest&max_errors_in_window=10&max_queue_length=1000&read_concurrency=0&subscription=s1&timeout=0ms&topics=persistent://public/default/pt-zgc' to database testdb with parser '{"global":{"cache":{"keep_days":"30d","max_size":"1GB","rotate_count":100,"location":"","on_fail":"skip"},"archive":{"keep_days":"30d","max_size":"1GB","rotate_count":100,"location":"","on_fail":"rotate"},"database_connection_error":"cache","database_not_exist":"break","table_not_exist":"retry","primary_timestamp_overflow":"archive","primary_timestamp_null":"archive","primary_key_null":"archive","table_name_length_overflow":"archive","table_name_contains_illegal_char":{"replace_to":""},"variable_not_exist_in_table_name_template":{"replace_to":""},"field_name_not_found":"add_field","field_name_length_overflow":"archive","field_length_extend":true,"field_length_overflow":"archive","ingesting_error":"archive","connection_timeout_in_second":"30s"},"parse":{"value":{"json":""}},"model":{"name":"r${id}","using":"meters","tags":["groupid","location"],"columns":["ts","id","v_str"]},"mutate":[{"map":{"ts":{"cast":"ts","as":"TIMESTAMP(ms)"},"id":{"cast":"id","as":"BIGINT"},"v_str":{"cast":"v_str","as":"VARCHAR"},"groupid":{"cast":"groupid","as":"BIGINT"},"location":{"cast":"location","as":"VARCHAR"}}}]}';

```

```
create xnode job on 1 with config '{"json":true}';

create xnode job on 1 with config '{"json":true}' status 'running';

show xnode jobs

drop xnode job 3;

```

 alter xnode job:

```
ALTER XNODE JOB 1 SET XNODE 1;
```

当前 alter 语句不完整：

```
taos> ALTER XNODE TASK 1 WITH via '10';

DB error: syntax error near "1 with via '10';" [0x80002600] (0.000323s)
taos> ALTER XNODE TASK '1' WITH via '10';
setXnodeTaskOption: k:"via", v:"'10';"

DB error: xnode task source and sink should not be NULL [0x80002600] (0.000195s)

```

```
alter xnode job `<jid>`  // 这个 jid 必须是 整数；
```



### pytest 测试

```
另外是 pytest 执行LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 pytest cases/42-Xnode/test_xnode.py -q 正常运行，但是运行几次后，突然再运行就报 connection error 之类的错误了，我看taosd什么都不启动了：

修复方法： 删除 taosd 使用的数据目录，在 sim 里：
(.venv) root@ha ~/zgc/TDengine/test (feat/agent-6646814636-main)$ rm -rf ../sim/*

再执行九成功了：
(.venv) root@ha ~/zgc/TDengine/test (feat/agent-6646814636-main)$ LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.8 pytest cases/42-Xnode/test_xnode.py -q

#debug 日志
 LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest  --log-cli-level=DEBUG  cases/42-Xnode/test_xnode.py -q 

```

执行 pytest 前，必须要:

```
make install -j20  
```

因为 pytest 调用的还是 /usr/local/taos/driver  里的



本地 docker:

最后使用 kill 杀死 taosd 可以看到退出的 sanitizer 的信息，用 ctrl + c 的方式不行：

```
 cd test
 source .venv/bin/activate
 LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest cases/42-Xnode/test_xnode.py -q
 
 LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest cases/42-Xnode/test_xnode.py::TestXnode::test_alter_token -q
 
 LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/42-Xnode/test_xnode.py::TestXnode::test_alter_token -q
```





#### TDengine 开发必须的 ci 用例

```
# 跑 error_code 的
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/81-Tools/01-Check/test_check_error_code.py -q


```





### 测试数据

```

/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic test-topic --partitions 5

/opt/kafka/bin/kafka-topics.sh --describe --topic abc --bootstrap-server localhost:9092


create xnode "192.168.2.158:6081" user root pass 'taosdata';



create xnode "localhost:6081" user root pass 'taosdata';
create xnode "localhost:6082";
create xnode "localhost:6083";
create database test;
create xnode task "t2" from 'kafka://localhost:9092?topics=abc&group=abcgroup' to 'taos+ws://localhost:6041/test' with parser '{"model":{"name":"cc_abc","using":"cc","tags":["g"],"columns":["ts","b"]},"mutate":[{"map":{"ts":{"cast":"ts","as":"TIMESTAMP(ms)"},"b":{"cast":"a","as":"VARCHAR"},"g":{"value":"1","as":"INT"}}}]}';
show xnode tasks\G;
start xnode task 1;


create xnode task "t3" from 'kafka://localhost:9092?topics=test&group=abcgroup' to 'taos+ws://localhost:6041/test' with parser '{"model":{"name":"cc_abc","using":"cc","tags":["g"],"columns":["ts","b"]},"mutate":[{"map":{"ts":{"cast":"ts","as":"TIMESTAMP(ms)"},"b":{"cast":"a","as":"VARCHAR"},"g":{"value":"1","as":"INT"}}}]}';


start xnode task 't1';

show xnode jobs;

stop xnode task 't1';

drop xnode task 't1';


```

```

taos> rebalance xnode job where age > 10 or age > 1 or name=1 or zgc=1 or abc=10;
xxxzgc *** where nodetype: 4, ast: {
        "NodeType":     "4",
        "Name": "LogicCondition",
        "LogicCondition":       {
                "DataType":     {
                        "Type": "0",
                        "Precision":    "0",
                        "Scale":        "0",
                        "Bytes":        "0"
                },
                "AliasName":    "2515744548289141832",
                "UserAlias":    "age > 10 or age > 1 or name=1 or zgc=1 or abc=10",
                "RelatedTo":    "0",
                "BindExprID":   "0",
                "CondType":     "2",
                "Parameters":   [{
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "12521370265370106441",
                                        "UserAlias":    "age > 10",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "40",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "age",
                                                        "UserAlias":    "age",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "age",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "4555470977590941194",
                                                        "UserAlias":    "10",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "2",
                                                        "Literal":      "10",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "1609527377948162648",
                                        "UserAlias":    "age > 1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "40",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "age",
                                                        "UserAlias":    "age",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "age",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "1503636689330325722",
                                        "UserAlias":    "name=1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "name",
                                                        "UserAlias":    "name",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "name",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "12438137017733867032",
                                        "UserAlias":    "zgc=1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "zgc",
                                                        "UserAlias":    "zgc",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "zgc",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "9372953878580161909",
                                        "UserAlias":    "abc=10",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "abc",
                                                        "UserAlias":    "abc",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "abc",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "4555470977590941194",
                                                        "UserAlias":    "10",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "2",
                                                        "Literal":      "10",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }]
        }
}
xxxzgc *** logic condition node: 5
xxxzgc *** node ast: {"NodeType":"4","Name":"LogicCondition","LogicCondition":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"2515744548289141832","UserAlias":"age > 10 or age > 1 or name=1 or zgc=1 or abc=10","RelatedTo":"0","BindExprID":"0","CondType":"2","Parameters":[{"NodeType":"3","Name":"Operator","Operator":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"12521370265370106441","UserAlias":"age > 10","RelatedTo":"0","BindExprID":"0","OpType":"40","Left":{"NodeType":"1","Name":"Column","Column":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"age","UserAlias":"age","RelatedTo":"0","BindExprID":"0","TableId":"0","TableType":"0","ColId":"0","ProjId":"0","ColType":"0","DbName":"","TableName":"","TableAlias":"","ColName":"age","DataBlockId":"0","SlotId":"0","TableHasPk":false,"IsPk":false,"NumOfPKs":"0","HasDep":false,"HasRef":false,"RefDb":"","RefTable":"","RefCol":"","IsPrimTs":false}},"Right":{"NodeType":"2","Name":"Value","Value":{"DataType":{"Type":"14","Precision":"0","Scale":"0","Bytes":"8"},"AliasName":"4555470977590941194","UserAlias":"10","RelatedTo":"0","BindExprID":"0","LiteralSize":"2","Literal":"10","Flag":false,"Translate":false,"NotReserved":false,"IsNull":false,"Unit":"0"}}}},{"NodeType":"3","Name":"Operator","Operator":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"1609527377948162648","UserAlias":"age > 1","RelatedTo":"0","BindExprID":"0","OpType":"40","Left":{"NodeType":"1","Name":"Column","Column":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"age","UserAlias":"age","RelatedTo":"0","BindExprID":"0","TableId":"0","TableType":"0","ColId":"0","ProjId":"0","ColType":"0","DbName":"","TableName":"","TableAlias":"","ColName":"age","DataBlockId":"0","SlotId":"0","TableHasPk":false,"IsPk":false,"NumOfPKs":"0","HasDep":false,"HasRef":false,"RefDb":"","RefTable":"","RefCol":"","IsPrimTs":false}},"Right":{"NodeType":"2","Name":"Value","Value":{"DataType":{"Type":"14","Precision":"0","Scale":"0","Bytes":"8"},"AliasName":"5001870860487857737","UserAlias":"1","RelatedTo":"0","BindExprID":"0","LiteralSize":"1","Literal":"1","Flag":false,"Translate":false,"NotReserved":false,"IsNull":false,"Unit":"0"}}}},{"NodeType":"3","Name":"Operator","Operator":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"1503636689330325722","UserAlias":"name=1","RelatedTo":"0","BindExprID":"0","OpType":"44","Left":{"NodeType":"1","Name":"Column","Column":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"name","UserAlias":"name","RelatedTo":"0","BindExprID":"0","TableId":"0","TableType":"0","ColId":"0","ProjId":"0","ColType":"0","DbName":"","TableName":"","TableAlias":"","ColName":"name","DataBlockId":"0","SlotId":"0","TableHasPk":false,"IsPk":false,"NumOfPKs":"0","HasDep":false,"HasRef":false,"RefDb":"","RefTable":"","RefCol":"","IsPrimTs":false}},"Right":{"NodeType":"2","Name":"Value","Value":{"DataType":{"Type":"14","Precision":"0","Scale":"0","Bytes":"8"},"AliasName":"5001870860487857737","UserAlias":"1","RelatedTo":"0","BindExprID":"0","LiteralSize":"1","Literal":"1","Flag":false,"Translate":false,"NotReserved":false,"IsNull":false,"Unit":"0"}}}},{"NodeType":"3","Name":"Operator","Operator":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"12438137017733867032","UserAlias":"zgc=1","RelatedTo":"0","BindExprID":"0","OpType":"44","Left":{"NodeType":"1","Name":"Column","Column":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"zgc","UserAlias":"zgc","RelatedTo":"0","BindExprID":"0","TableId":"0","TableType":"0","ColId":"0","ProjId":"0","ColType":"0","DbName":"","TableName":"","TableAlias":"","ColName":"zgc","DataBlockId":"0","SlotId":"0","TableHasPk":false,"IsPk":false,"NumOfPKs":"0","HasDep":false,"HasRef":false,"RefDb":"","RefTable":"","RefCol":"","IsPrimTs":false}},"Right":{"NodeType":"2","Name":"Value","Value":{"DataType":{"Type":"14","Precision":"0","Scale":"0","Bytes":"8"},"AliasName":"5001870860487857737","UserAlias":"1","RelatedTo":"0","BindExprID":"0","LiteralSize":"1","Literal":"1","Flag":false,"Translate":false,"NotReserved":false,"IsNull":false,"Unit":"0"}}}},{"NodeType":"3","Name":"Operator","Operator":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"9372953878580161909","UserAlias":"abc=10","RelatedTo":"0","BindExprID":"0","OpType":"44","Left":{"NodeType":"1","Name":"Column","Column":{"DataType":{"Type":"0","Precision":"0","Scale":"0","Bytes":"0"},"AliasName":"abc","UserAlias":"abc","RelatedTo":"0","BindExprID":"0","TableId":"0","TableType":"0","ColId":"0","ProjId":"0","ColType":"0","DbName":"","TableName":"","TableAlias":"","ColName":"abc","DataBlockId":"0","SlotId":"0","TableHasPk":false,"IsPk":false,"NumOfPKs":"0","HasDep":false,"HasRef":false,"RefDb":"","RefTable":"","RefCol":"","IsPrimTs":false}},"Right":{"NodeType":"2","Name":"Value","Value":{"DataType":{"Type":"14","Precision":"0","Scale":"0","Bytes":"8"},"AliasName":"4555470977590941194","UserAlias":"10","RelatedTo":"0","BindExprID":"0","LiteralSize":"2","Literal":"10","Flag":false,"Translate":false,"NotReserved":false,"IsNull":false,"Unit":"0"}}}}]}}, astLen: 5270

DB error: Message not processed [0x80000121] (0.005177s)




taos> rebalance xnode job where 1=1;
xxxzgc *** where nodetype: 3, ast: {
        "NodeType":     "3",
        "Name": "Operator",
        "Operator":     {
                "DataType":     {
                        "Type": "0",
                        "Precision":    "0",
                        "Scale":        "0",
                        "Bytes":        "0"
                },
                "AliasName":    "1993694102546043161",
                "UserAlias":    "1=1",
                "RelatedTo":    "0",
                "BindExprID":   "0",
                "OpType":       "44",
                "Left": {
                        "NodeType":     "2",
                        "Name": "Value",
                        "Value":        {
                                "DataType":     {
                                        "Type": "14",
                                        "Precision":    "0",
                                        "Scale":        "0",
                                        "Bytes":        "8"
                                },
                                "AliasName":    "5001870860487857737",
                                "UserAlias":    "1",
                                "RelatedTo":    "0",
                                "BindExprID":   "0",
                                "LiteralSize":  "1",
                                "Literal":      "1",
                                "Flag": false,
                                "Translate":    false,
                                "NotReserved":  false,
                                "IsNull":       false,
                                "Unit": "0"
                        }
                },
                "Right":        {
                        "NodeType":     "2",
                        "Name": "Value",
                        "Value":        {
                                "DataType":     {
                                        "Type": "14",
                                        "Precision":    "0",
                                        "Scale":        "0",
                                        "Bytes":        "8"
                                },
                                "AliasName":    "5001870860487857737",
                                "UserAlias":    "1",
                                "RelatedTo":    "0",
                                "BindExprID":   "0",
                                "LiteralSize":  "1",
                                "Literal":      "1",
                                "Flag": false,
                                "Translate":    false,
                                "NotReserved":  false,
                                "IsNull":       false,
                                "Unit": "0"
                        }
                }
        }
}



```

```
这个 语句的 `NULL` 竟然被定为 column ? 这种转义可以的！

taos> rebalance xnode job where xnode_id!=`NUll`;
xxxzgc *** node ast: {
        "NodeType":     "3",
        "Name": "Operator",
        "Operator":     {
                "DataType":     {
                        "Type": "0",
                        "Precision":    "0",
                        "Scale":        "0",
                        "Bytes":        "0"
                },
                "AliasName":    "15451760794959550287",
                "UserAlias":    "xnode_id!=`NUll",
                "RelatedTo":    "0",
                "BindExprID":   "0",
                "OpType":       "45",
                "Left": {
                        "NodeType":     "1",
                        "Name": "Column",
                        "Column":       {
                                "DataType":     {
                                        "Type": "0",
                                        "Precision":    "0",
                                        "Scale":        "0",
                                        "Bytes":        "0"
                                },
                                "AliasName":    "xnode_id",
                                "UserAlias":    "xnode_id",
                                "RelatedTo":    "0",
                                "BindExprID":   "0",
                                "TableId":      "0",
                                "TableType":    "0",
                                "ColId":        "0",
                                "ProjId":       "0",
                                "ColType":      "0",
                                "DbName":       "",
                                "TableName":    "",
                                "TableAlias":   "",
                                "ColName":      "xnode_id",
                                "DataBlockId":  "0",
                                "SlotId":       "0",
                                "TableHasPk":   false,
                                "IsPk": false,
                                "NumOfPKs":     "0",
                                "HasDep":       false,
                                "HasRef":       false,
                                "RefDb":        "",
                                "RefTable":     "",
                                "RefCol":       "",
                                "IsPrimTs":     false
                        }
                },
                "Right":        {
                        "NodeType":     "1",
                        "Name": "Column",
                        "Column":       {
                                "DataType":     {
                                        "Type": "0",                 // 这里 0 就是 NULL 的判断，是 taos.h 里定义的：TSDB_DATA_TYPE_NULL
                                        "Precision":    "0",
                                        "Scale":        "0",
                                        "Bytes":        "0"
                                },
                                "AliasName":    "NUll",
                                "UserAlias":    "NUll",
                                "RelatedTo":    "0",
                                "BindExprID":   "0",
                                "TableId":      "0",
                                "TableType":    "0",
                                "ColId":        "0",
                                "ProjId":       "0",
                                "ColType":      "0",
                                "DbName":       "",
                                "TableName":    "",
                                "TableAlias":   "",
                                "ColName":      "NUll",
                                "DataBlockId":  "0",
                                "SlotId":       "0",
                                "TableHasPk":   false,
                                "IsPk": false,
                                "NumOfPKs":     "0",
                                "HasDep":       false,
                                "HasRef":       false,
                                "RefDb":        "",
                                "RefTable":     "",
                                "RefCol":       "",
                                "IsPrimTs":     false
                        }
                }
        }
}, astLen: 1668

```

## 问题

1. 有时候需要重新 编辑一下 .h 头问题的内容，才会重新建索引，vscode 才不会报错
2. 如果有改任何的 CMakeLists.txt ，就需要先 sh build.sh gen 以下，先产生新的配置文件！
3. bnode 什么时候发送消息创建 mqtt 进程的？

## 有用的代码

ast 的处理代码: 核心是 nodesStringToNode() 和

```
static int32_t processAst(SMqTopicObj *topicObj, const char *ast) {
  SNode *     pAst = NULL;
  SQueryPlan *pPlan = NULL;
  int32_t     code = TSDB_CODE_SUCCESS;
  int32_t     lino = 0;

  PRINT_LOG_START
  if (ast == NULL) {
    topicObj->physicalPlan = taosStrdup("");
    goto END;
  }
  qDebugL("%s topic:%s ast %s", __func__, topicObj->name, ast);
  MND_TMQ_RETURN_CHECK(nodesStringToNode(ast, &pAst));
  MND_TMQ_RETURN_CHECK(qExtractResultSchema(pAst, &topicObj->schema.nCols, &topicObj->schema.pSchema));

  SPlanContext cxt = {.pAstRoot = pAst, .topicQuery = true};
  MND_TMQ_RETURN_CHECK(qCreateQueryPlan(&cxt, &pPlan, NULL));
  if (pPlan == NULL) {
    code = TSDB_CODE_MND_INVALID_TOPIC_QUERY;
    goto END;
  }
  int32_t levelNum = LIST_LENGTH(pPlan->pSubplans);
  if (levelNum != 1) {
    code = TSDB_CODE_MND_INVALID_TOPIC_QUERY;
    goto END;
  }

  SNodeListNode *pNodeListNode = (SNodeListNode *)nodesListGetNode(pPlan->pSubplans, 0);
  MND_TMQ_NULL_CHECK(pNodeListNode);
  int32_t opNum = LIST_LENGTH(pNodeListNode->pNodeList);
  if (opNum != 1) {
    code = TSDB_CODE_MND_INVALID_TOPIC_QUERY;
    goto END;
  }

  code = nodesNodeToString(nodesListGetNode(pNodeListNode->pNodeList, 0), false, &topicObj->physicalPlan, NULL);

END:
  nodesDestroyNode(pAst);
  qDestroyQueryPlan(pPlan);
  PRINT_LOG_END
  return code;
}
```

## 开发总结：

1. show statement 里的内容不可以放到下面，要和 sysTableShowAdapter 里的定义一致。

```
  // the order of QUERY_NODE_SHOW_* must be aligned with the order of `sysTableShowAdapter` defines.
  QUERY_NODE_SHOW_TOKENS_STMT,
  QUERY_NODE_SHOW_ENCRYPT_STATUS_STMT,
  QUERY_NODE_SHOW_ROLES_STMT,
  QUERY_NODE_SHOW_ROLE_PRIVILEGES_STMT,
  QUERY_NODE_SHOW_ROLE_COL_PRIVILEGES_STMT,
  QUERY_NODE_SHOW_XNODES_STMT,
  QUERY_NODE_SHOW_XNODE_TASKS_STMT,
  QUERY_NODE_SHOW_XNODE_AGENTS_STMT,
  QUERY_NODE_SHOW_XNODE_JOBS_STMT,
```

2. community 的编译版本要和 TDinternal 的版本对应上
3. void检查：

   ```
   ./test/ci/check_void.sh -c test/ci/func.txt -f source/

4.  日志里的编译指令:

   主要是：   

   ```
   cmake .. -DBUILD_TEST=true -DBUILD_HTTPS=false -DBUILD_TOOLS=true -DCMAKE_BUILD_TYPE=Release && make -j10
   ```

   

   ```

   2025-12-30T17:47:39.5906250Z Running command: date (cwd=/Users/taos/actions-runner/_work/TDengine/TDengine/.github/scripts)
   2025-12-30T17:47:39.6005850Z Wed Dec 31 01:47:27 CST 2025
   2025-12-30T17:47:39.6106890Z Running command: rm -rf debug && mkdir debug && cd /var/lib/jenkins/workspace/TDinternal/debug (cwd=/var/lib/jenkins/workspace/TDinternal)
   2025-12-30T17:47:39.6207980Z Running command: echo $PATH (cwd=/Users/taos/actions-runner/_work/TDengine/TDengine/.github/scripts)
   2025-12-30T17:47:39.6309800Z /usr/local/opt/openjdk/bin:/usr/local/opt/openjdk/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/go/bin:/usr/local/go//bin
   2025-12-30T17:47:39.6411670Z Running command: echo "PATH=/opt/homebrew/bin:$PATH" >> $GITHUB_ENV (cwd=/Users/taos/actions-runner/_work/TDengine/TDengine/.github/scripts)
   2025-12-30T17:47:39.6511130Z Running command: cmake .. -DBUILD_TEST=true -DBUILD_HTTPS=false -DBUILD_TOOLS=true -DCMAKE_BUILD_TYPE=Release && make -j10 (cwd=/var/lib/jenkins/workspace/TDinternal/debug)
   2025-12-30T17:47:39.6610600Z -- The C compiler identification is AppleClang 12.0.0.12000032

   ```

5. 这个分支放了一些 rust 的项目的配置：feat/zgc-xnode-syntax-mock-xnoded， xnoded 是配置在 CMakefiles.txt 里的
6. cases.task 的是配置  py 脚本的测试用例, 这块的测试用例主要是集成测试; pytest 的测试用例在 test/cases 下

8. 内存泄漏的形式：
   1. free(&name);  但是 name 是一个 char *name; 释放的地址就不对
   2. 遗漏泄漏：指针数组：char *arr;  arr = calloc(10, sizeof(char *)); 结果只只放了 free(arr[0]); free(arr[1]); ... free(arr[N]); 忘了释放 arr: free(arr); 只释放了内容，没有释放容器！
   3. 排查问题：test_xnode.py 的问题，只能一条一条的测试，二分法;  知道哪条 sql 后, 才能顺着入口 sql 继续梳理逻辑，用 gdb 单步调试排除
   4.  cmd::= 顶级语法的报的语法错误，就需要释放资源：比如 with 等的资源，可能出错了，就没有再执行 资源释放了
9. 序列化都是 len 个字节，但是反序列化的时候，需要加 +1 作为





9. xnoded 被 systemd 管理的问题：

   xnoded 被 taosd 启动，但是都被 systemd 管理，所以如果 systemd 发送 kill 命令给进程组 id: `kill -TERM -<PGID>`  那么，xnoded 和 taosd 都会同时收到 kill 信号，但是 xnoded 退出快， taosd 走退出流程慢。xnoded 退出后，会被 taosd 马上拉起来，所以启动了新的进程；同时 taosd 杀死的是老的 xnoded 进程, 老的 xnoded 已经退出了，新的还没有创建，还没被 libuv 管理，所以很可能错开了，或者 老的退出逻辑卡死了，目前看老的退出进程卡死了：

   ```
   void xnodeMgmtStopXnoded(void) {
     SXnodedData *pData = &xnodedGlobal;
     xndInfo("stopping xnoded, need cleanup:%d, spawn err:%d, isStopped:%d", pData->needCleanUp, pData->spawnErr, atomic_load_32(&pData->isStopped));
     if (!pData->needCleanUp || atomic_load_32(&pData->isStopped)) {
       return;
     }
     atomic_store_32(&pData->isStopped, 1);
     pData->needCleanUp = false;
     xndInfo("xxxzgc ******* xnoded is cleaned up   111");
     (void)uv_process_kill(&pData->process, SIGTERM);
     xndInfo("xxxzgc ******* xnoded is cleaned up  2222222");
     uv_barrier_destroy(&pData->barrier);                              // 这里以后就没有输出了
     xndInfo("xxxzgc ******* xnoded is cleaned up  333333");
   
     if (uv_thread_join(&pData->thread) != 0) {
       xndError("stop xnoded: failed to join xnoded thread");
     }
     xndInfo("xnoded is cleaned up");
   
     pData->isStarted = false;
   
     return;
   }
   ```

   

   ```
   ps -eo pid,ppid,pgid,comm | grep your_process
   
   ps -eo pid,ppid,pgid,comm | grep taosd
   ps -eo pid,ppid,pgid,comm | grep xnoded
   ```



systemctl stop 会杀死所有的在 cgroup 里的所有进程：

```
使用 systemctl stop 时行为完全不同！systemd 会管理服务整个控制组（cgroup）中的所有进程。
systemctl stop 的默认行为


子进程一定会被杀死的原因
systemd 使用 cgroup 来管理服务的所有进程：
复制
/sys/fs/cgroup/systemd/system.slice/your-service.service/
├── cgroup.procs    # 包含该服务所有进程的 PID
└── ...
当执行 systemctl stop 时：
systemd 扫描 cgroup 中的所有进程
逐个发送信号，确保没有孤儿进程存活
```

