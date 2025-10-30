todo:

1. agent 发送流程： persist-component 组件使用了 breakpoint db
2.

测试用例:

```
cargo test --package taosx-core --lib -- utils::breakpoints::tests --nocapture
```



influxdb 导入数据

```

influx --username root --password taosdata -database mydb -execute 'SELECT * FROM scada_value WHERE time>=now()-14d' -format column -precision ns > scada_value.lp


influx -import -path=output_lp.csv -precision=ns -password=taosdata  -username=root
```
