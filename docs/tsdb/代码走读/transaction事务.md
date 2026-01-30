



1. pReq 传 NULL, 会等待，因为底层没有数据带给 RPC 框架，有数据给了 RPC 框架，才会返回请求
2. 函数返回 ACTION_IN_PROGRESS 才会等 事务执行完成，才会返回数据
3. 如果 code = 0 直接返回了，那么底层就会出现报出现 冗余消息 的错误
4. RPC 根据是 ID 来进行数据确认，底层是全双工的！

5. *pReq 可以传 NULL, 传 NULL 就不回返回数据了：

```
STrans *mndTransCreate(SMnode *pMnode, ETrnPolicy policy, ETrnConflct conflict, const SRpcMsg *pReq,
                       const char *opername) {
                       ...
}
```



测试发现：

1. 事务先成功了，那就会让 rpc 先返回， taos shell 也就返回了
2. 如果函数先返回了，且状态码是 TSDB_CODE_ACTION_IN_PROGRESS, 就会等事务。否则也会先返回
3. 事务会排队执行！所以需要返回数据的事务，放到最后执行！

场景：

1. 函数返回 TSDB_CODE_ACTION_IN_PROGRESS, 事务传 NULL， 会马上返回吗？ 看样子会是卡死！验证了：会卡死！！
2. 函数返回 0, 事务传 not NULL 非空，不回等事务结束，会返回，但是 show 等命令不回马上查询出来！!
3. 返回返回 0  和 事务先结束，哪一个先返回值给 RPC， 就会马上返回给客户端！！



transaction 是 2 阶段提交，主要会为了一致性，也主要是给 元数据同步 使用；data 数据副本是异步 copy，并非使用二阶段提交，使用二阶段提交数据性能当然会相当的慢！！





一种场景：

1. 如果 pReq 传 NULL, code 返回 0， 对端  是会等待的（这里有待确认，正确的逻辑是会返回，但是报 冗余消息 的错误）

2. 如果 pReq 传 NULL, code 返回 TSDB_CODE_ACTION_IN_PROGRESS ， 对端 是会等待的，事务执行完，没有数据，RPC 会一直等待数据

3. 如果 pReq 传 not NULL, code 返回 TSDB_CODE_ACTION_IN_PROGRESS ， 等待事务执行完会返回

4. 如果 pReq 传 not NULL, code 返回 0 ， 不会等待事务执行完，会马上返回，但是事务执行完再返回就报 冗余消息 的错误

   



