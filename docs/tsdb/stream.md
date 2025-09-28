tsdb 关键字 和 占位符 区别开：

占位符： _wstart, 

流占位符：_twstart

加 %% 前缀的占位符：

- %%trows：只能用于 FROM 子句，在使用 %%trows 的语句中不支持 where 条件过滤，不支持对 %%trows 进行关联查询。
- %%tbname：可以用于 FROM、SELECT 和 WHERE 子句。
- 其他占位符：只能用于 SELECT 和 WHERE 子句。





关键字：

tbname, now, 1s 

WINDOW_CLOSE 和 WINDOW_OPEN 都是





refer 流计算：

https://docs.taosdata.com/reference/taos-sql/stream/





表的关键字：

```
tbname: 
```

流计算：

```
WINDOW_CLOSE
```



## 流计算相关语法

```
drop database if exists `zgc_test1`;
drop database if exists `zgc_test2`;
create database if not exists `zgc_test1`;
create database if not exists `zgc_test2`;
use `zgc_test1`;
drop stream if exists `zgc_test_stream`;
create table `zgc_test1`.`meters`(ts timestamp, val int) tags(id int);

create stream `zgc_test_stream` state_window(val) from `zgc_test1`.meters  partition by tbname into `zgc_test1`.`zgc_test_stream` as select _twstart,tbname,avg(val) from %%trows;

insert into t1 using meters tags(1) values(now, 11),(now+1s,11),(now+2s,11),(now+3s,10),(now+4s,10),(now+5s,10);
insert into t1 using meters tags(1) values(now, 12);

insert into t2 using meters tags(2) values(now, 22),(now+1s,22),(now+2s,22),(now+3s,21),(now+4s,21),(now+5s,21);
```
3.3.8.x 后支持 cast:
```
create stream `zgc_test_stream` state_window(cast(val as int)) from `zgc_test1`.meters partition by tbname into `zgc_test1`.`zgc_test_stream` as select _twstart,tbname,avg(val) from %%trows;

insert into t1 using meters tags(1) values(now, 11.1),(now+1s,11.2),(now+2s,11.3),(now+3s,10.1),(now+4s,10.2),(now+5s,10.3);\
insert into t2 using meters tags(2) values(now, 22.1),(now+1s,22.2),(now+2s,22.3),(now+3s,21.1),(now+4s,21.2),(now+5s,21.3);\
insert into t3 using meters tags(3) values(now, 33.1),(now+1s,33.2),(now+2s,33.3),(now+3s,32.1),(now+4s,32.2),(now+5s,32.3);\
insert into t4 using meters tags(4) values(now, 44.1),(now+1s,44.2),(now+2s,44.3),(now+3s,43.1),(now+4s,43.2),(now+5s,43.3);\
insert into t5 using meters tags(5) values(now, 55.1),(now+1s,55.2),(now+2s,55.3),(now+3s,54.1),(now+4s,54.2),(now+5s,55.3);\
```
