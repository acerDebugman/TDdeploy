



libuv 调研：

```
```



## `uv_stop` - 停止事件循环

**作用**：请求事件循环（`uv_run`）**尽快返回**，停止阻塞





**工作原理**：

- 设置内部标志位 `stop_flag = 1`
- 唤醒事件循环（如果正在阻塞在 epoll/kqueue/IOCP）
- **不会立即停止**：当前正在执行的回调会完成，但不再等待新的事件

**使用场景**：

- 收到 SIGINT 信号后优雅退出
- 完成特定任务后停止循环（如处理完 N 个请求）



问题：

uv_loop_close, uv_stop, uv_close 怎么使用？有什么区别？

| 函数                | 操作对象    | 作用                                       | 调用时机               | 线程安全                 |
| ------------------- | ----------- | ------------------------------------------ | ---------------------- | ------------------------ |
| **`uv_stop`**       | 事件循环    | 设置停止标志，让 `uv_run()` 尽快返回       | 需要退出循环时         | ✅ 是                     |
| **`uv_close`**      | 单个 Handle | 关闭具体资源（timer/tcp/file等），触发回调 | 不再需要某个 handle 时 | ❌ 否（必须在 loop 线程） |
| **`uv_loop_close`** | 整个 Loop   | 销毁 loop，检查所有资源已释放              | 程序退出前             | ❌ 否（必须在 loop 线程） |



记住口诀：**Stop 是刹车，Close 是关门，Loop Close 是拆房**。先刹车停稳，再关所有门，最后才能拆房。

关闭顺序：uv_stop -> uv_close -> uv_loop_close 



```
static void xnodeMgmtXnodedCloseWalkCb(uv_handle_t *handle, void *arg) {
  TAOS_XNODED_MGMT_CHECK_PTR_RVOID(handle);
  if (!uv_is_closing(handle)) {
    xndDebug("xnoded closing handle type:%d, ptr:%p", handle->type, handle);
    uv_close(handle, NULL);
  }
}

static void xnodeMgmtXnodedStopAsyncCb(uv_async_t *async) {
  TAOS_XNODED_MGMT_CHECK_PTR_RVOID(async);
  SXnodedData *pData = async->data;
  uv_stop(&pData->loop);
}

static void xnodeMgmtWatchXnoded(void *args) {
  TAOS_XNODED_MGMT_CHECK_PTR_RVOID(args);
  SXnodedData *pData = args;
  TAOS_UV_CHECK_ERRNO(uv_loop_init(&pData->loop));
  TAOS_UV_CHECK_ERRNO(uv_async_init(&pData->loop, &pData->stopAsync, xnodeMgmtXnodedStopAsyncCb));
  pData->stopAsync.data = pData;
  TAOS_UV_CHECK_ERRNO(xnodeMgmtSpawnXnoded(pData));
  atomic_store_32(&pData->spawnErr, 0);
  (void)uv_barrier_wait(&pData->barrier);
  int32_t num = uv_run(&pData->loop, UV_RUN_DEFAULT);
  xndInfo("xnoded loop exit with %d active handles, line:%d", num, __LINE__);

  uv_walk(&pData->loop, xnodeMgmtXnodedCloseWalkCb, NULL);
  num = uv_run(&pData->loop, UV_RUN_DEFAULT);
  xndInfo("xnoded loop exit with %d active handles, line:%d", num, __LINE__);
  if (uv_loop_close(&pData->loop) != 0) {
    xndError("xnoded loop close failed, lino:%d", __LINE__);
  }
  return;

_exit:
  if (terrno != 0) {
    (void)uv_barrier_wait(&pData->barrier);
    atomic_store_32(&pData->spawnErr, terrno);
    if (uv_loop_close(&pData->loop) != 0) {
      xndError("xnoded loop close failed, lino:%d", __LINE__);
    }

    xndError("xnoded thread exit with code:%d lino:%d", terrno, __LINE__);
    terrno = TSDB_CODE_XNODE_UV_EXEC_FAILURE;
  }
}
```





