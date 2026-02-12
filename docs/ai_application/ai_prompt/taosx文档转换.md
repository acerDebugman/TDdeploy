





```
在 old_arch 里有单节点的 taosx 数据管道项目的 RS, FS, DS 文档描述信息，经过高可用改造，目前已经变成分布式的高可用版本，在 new_arch 目录有新版本的 DS 描述，当前 taosx 新版本的具体代码在 /app/taosx 里。

你现在的任务是：结合新版本的文档描述 new_arch/taosx高可用-DS.md 和 new_arch/taosx高可用新架构补充描述.md，以及 rs_fs_ds_templates 里的对应的 RS,FS,DS 对应文档模板，改写为新的 RS,FS,DS 文件，并输出到 output 目录里。
具体文档对应关系：
1. old_arch/数据管道-RS.md 参照模板 rs_fs_ds_templates/RS.md，改写为：output/新数据管道-RS.md
2. old_arch/数据管道-FS.md 参照模板 rs_fs_ds_templates/FS.md，改写为：output/新数据管道-FS.md
3. old_arch/数据管道-DS.md 参照模板 rs_fs_ds_templates/DS.md，改写为：output/新数据管道-DS.md

```







taosx 高可用新架构补充描述：

1. taosx 已经组件已经集成到 TDengine 平台，变成 TDengine 平台的数据同步项目，并且重命名为 xnode 组件。且通过 SQL 进行节点的管理，管理的相关命令在页面 https://docs.taosdata.com/reference/taos-sql/datain/ 上可以找到。
2. xnode 的高可用通过将节点元信息， 任务元信息写入到 taosd 的 mnode 中，mnode 中的 sdb 具有高可用的存储功能，可以在  taosd 集群中保持一致性。taosd 在 mnode 的 leader 节点再启动 xnoded 守护进程，xnoded 守护进程与 xnode 通信。在 xnoded 守护进程里管理 taosx 节点状态，分派任务，将任务根据具体数据源类型进行分片，并将分片任务指派到不同的 xnode 中执行，然后管理分片任务状态。通过这种方式实现 taosx 的高可用管理。
3. taosx-agent 的处理逻辑与旧版本相同，只是同样在 taosd 侧通过 sql 命令进行管理。taosx-agent 可与任何 xnode 通信，处理具体数据任务。





