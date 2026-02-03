

TDinternal 环境安装：

```
apt install python3.10-venv

cd /app/TDinternal/community/test

python3 -m venv .venv

# 激活
source .venv/bin/activate

使用 uv 安装，不使用 pip3 安装 pip3 install -r requirements.txt 
uv 安装：
uv pip install -r requirements.txt


cd /app/TDinternal/community/test
pytest cases/42-Xnode/test_xnode.py -q

# 显示 debug 打印信息：
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/42-Xnode/test_xnode.py -q

只跑某个函数：
pytest cases/42-Xnode/test_xnode.py::TestXnode::test_show_primitives -q

如果改了 pytest 代码没有生效，注意删除 pytest 的缓存：
cd /app/TDinternal/community/test
rm -rf .pytest_cache/

或者执行：pytest --cache-clear
```





```
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/42-Xnode/test_xnode.py -q

LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libasan.so.6 pytest --log-cli-level=DEBUG cases/42-Xnode/test_xnode.py::TestXnode::test_xnode_column_length -q
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

