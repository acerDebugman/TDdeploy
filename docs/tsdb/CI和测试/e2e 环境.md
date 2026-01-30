



```
执行一次，将 git 账号保存下来：
git config --global credential.helper store
git clone https://<token>@github.com/taosdata/TestNG_taosX.git /tmp/test
再执行：
poetry install --no-root
```



环境准备：

```
cp setenv.sh.example setenv.sh
# Change HOST to your test env ip or fqdn. We can just use localhost for local test.
source setenv.sh
```

执行测试：

```
poetry run pytest /path/to/test_function/tmq_test.py::test_sanity

# 跑所有 case
poetry run pytest test_function/csv_test.py

# 只跑某个 case 
poetry run pytest test_function/csv_test.py::test_sanity_csv_td32576_01

poetry run pytest --log-cli-level=DEBUG test_function/csv_test.py::test_sanity_csv_td32576_01
```



poetry 改为开发模式，从新安装：

```
testng-taosx = { git = "../../../TestNG_taosX", branch = "framework-only", develop = true }

poetry install --no-root
```

临时直接使用这种方式：

```
poetry add /app/TestNG_taosX --editable

poetry add /root/zgc/TestNG_taosX --editable
```



鉴权的问题, 现在 explorer 上的接口都需要加上用户名密码：

```
curl -u root:taosdata http://localhost:6060/api/x/tasks
```





指定 unix socket 访问, 查看状态：

```
curl --unix-socket /var/lib/taos.taosxnoded.sock http://localhost/xnode/1
```



 依赖 taosx 启动：

```
nohup taosx 2>&1 &
create xnode 'localhost:6055';
```



重新进入运行：

```
root@ha ~/taosx/tests/e2e (main)$ cp setenv.sh.example setenv.sh
root@ha ~/taosx/tests/e2e (main)$ source setenv.sh
root@ha ~/taosx/tests/e2e (main)$ poetry run pytest test_function/csv_test.py::test_sanity_csv_td32576_01
```













