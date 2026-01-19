

```
你现在在 TDengine 数据库的代码库的测试用例目录里，这里主要测试 xnode 相关的 sql 语法，现在这 1 条 sql 语法：
DROP XNODE JOB jid;
扩展支持了 where 语句变成:
DROP XNODE JOB [WHERE condition];
约束:
1. 目前支持的 where_clause 的布尔表达式不支持函数，只支持简单的逻辑运算: and,or,not; 比较运算支持 >,>=,<,<=,=,!=
示例: drop xnode job where id > 1;

下面是 drop 命令 where 条件可以操作的表字段信息：
job 表：
             field              |          type          |   length    |        note        |
=============================================================================================
 id                             | INT                    |           4 |                    |
 task_id                        | INT                    |           4 |                    |
 config                         | VARCHAR                |        4096 |                    |
 via                            | INT                    |           4 |                    |
 xnode_id                       | INT                    |           4 |                    |
 status                         | VARCHAR                |          16 |                    |
 reason                         | VARCHAR                |        1024 |                    |
 create_time                    | TIMESTAMP              |           8 |                    |
 update_time                    | TIMESTAMP              |           8 |                    |

你的任务是，仿照 test_xnode.py 里的函数，在里面添加几个测试 where 条件的测试用例函数, 注意自己使用 create xnode job on task_id with 语法添加数据，可以参照已经有的添加语句。你不用测试，只要给我写好测试用例的代码就行。
```
