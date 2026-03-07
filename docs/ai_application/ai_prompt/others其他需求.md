









```
当前 explorer 的 系统管理->用户->新增 操作里，做如下操作：
1. 新建用户 b，并且勾选主题 topic, 保存
2. 编辑用户 b, 当前页面并未显示已经保存的勾选的主题

经过我的排查确认：
1. 数据库是有数据的
2. 出错发送的 sql 是：GRANT subscribe ON `` to `b`
3. 出错页面应该在 explorer/src/views/8_administrator/views/components/userForm/index.vue


问题应该出在前端页面的执行逻辑上，继续排查并且修复这个问题，再注意看看页面逻辑上是否还有类似的问题
```





```
参考 explorer/tests 里的测试用例，添加 用户管理，
```



