



```
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest cases/42-Xnode/test_xnode.py -q
```





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


LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/42-Xnode/test_xnode.py::TestXnode::test_alter_token -q
	


LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/17-DataSubscription/02-Consume/test_tmq_vnode_split_dup_no_wal.py -N 3 --replica 3
```



## 清理脚本

```
#!/bin/bash

ps -ef | grep taosd | grep -v grep | awk '{print $2}' | xargs -I{} kill -9 {}
rm -rf /app/TDinternal/sim/*

```

