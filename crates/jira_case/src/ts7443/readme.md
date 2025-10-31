todo:

1. agent 发送流程： persist-component 组件使用了 breakpoint db
2. agent 模式下 TaskScheduler 的也要替换



测试用例:

```
cargo test --package taosx-core --lib -- utils::breakpoints::tests --nocapture
```



influxdb 导入数据

```

influx --username root --password taosdata -database mydb -execute 'SELECT * FROM scada_value WHERE time>=now()-14d' -format column -precision ns > scada_value.lp


influx -import -path=output_lp.csv -precision=ns -password=taosdata  -username=root
```




任务的状态:

1. 启动: start:
2. 重启: 没有 restart, 只有 stop -> start
3. 暂停: stop
4. 删除:


暂停任务调用: 

POST

```
http://192.168.2.131:6060/api/x/tasks/2/stop


```

启动任务调用:

POST

```
http://192.168.2.131:6060/api/x/tasks/2/start
```

执行逻辑:

1. task controller,  从 api 调用接口中传递,
2. 使用 `task_controller` 启动任务, 先从 sqlite 中获取 任务详情, 再使用 task.load_breakpoints() , 这方法调用 breakpoint_get_all 从 sled 中获取所有的 breakpoints 断点信息
3. 


**删除任务:**

post

```
http://192.168.2.131:6060/api/x/tasks/2/delete

```

执行逻辑:

1. scheduler 停止
2. 软删除 sqlite 数据
3. 最后删除 breakpoints
   ```
   taosx_core::utils::breakpoints::breakpoints_clear(&task_id).await?;
   ```


TaskScheduler 里的 table_cache 和 breakpoints_db 都是为了放到了 transofrm/sink 阶段的数据流 里去 写入数据 的. 这是为了


## 保存版本

修改了的将 breakpoint db 提为全局 cache 的版本:

fix/TS-7443-main-breakpoint-global-cache-version


新使用的组件:

```
once_cell  : 初始化执行
temp_env   : 用于测试环境,让修改同一个环境变量变成顺序的执行的,因为 tokio::test 是并行执行的,使用 temp_env 的 async 方法,可以顺序执行


```
