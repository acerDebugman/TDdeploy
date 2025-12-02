


```
使用 libuv 库给我写一个使用例子，一端是c语言编写，一端是 rust 编写，rust 这块使用 libuv 的 uv_listen 启动监听，c 侧创建链接，并且发送消息



```




开发：

1. 使用 libuv 进行任务管理，发送消息等
2. 测试 bnode，试一试 mqtt (低优先级)
3. 


todo:

1. mqtt发送
2. xnode 启动，目前现在初始化 open 的时候启动，后续改为 mnode 切换的时候启动，(先最小实现, 熟悉后再改)
3. 


todo:

1. 测试 libuv 和 调用集成
   1. 测试 c 与  rust 使用 libuv 通信
   2. 
2. 



当前 bnode 是通过：

1. libuv 管理
2. bnode 的功能是创建一个 mqtt 协议的服务，使 tsdb 的 topic 支持 mqtt, 其他的 mqtt 客户端可以直接连接 tsdb 的 mqtt 地址，直接获取数据工作
3. libuv 是一个类似 tokio 的异步io 库，但是没有 tokio 的抽象好
4. 




目录：

```
include/libs/txnode/txnode.h   // 用于开放头文件
source/dnode/mgmt/mgmt_xnode/CMakeLists.txt  // 启动的时候使用, 包装为主函数
source/dnode/mgmt/mgmt_xnode/inc/xndInt.h
source/dnode/mgmt/mgmt_xnode/src/xnode.c
source/dnode/xnode/src/xnode.c               // 启动函数等模块
source/libs/txnode/inc/txnodeInt.h           // 具体与 xnoded 通信模块 txnode 内容
source/libs/txnode/mgmt/CMakeLists.txt
source/libs/txnode/mgmt/src/txnodeMgmt.c      // txnode 管理的
source/libs/txnode/xnode/src/txnodeDaemon.c    // 启动的任务的，libuv 管理的话，应该是启动的线程
```


整体流程：

dmMain() -> dmInit() -> dmInitDnode() -> 


## 测试

1. 创建 bnode
   ```
   create bnode on dnode 1 protocol 'mqtt';
   ```



环境安装

```
apt install clang clangd lldb cmake
```


## 问题

1. 有时候需要重新 编辑一下 .h 头问题的内容，才会重新建索引，vscode 才不会报错
2. 如果有改任何的 CMakeLists.txt ，就需要先 sh build.sh gen 以下，先产生新的配置文件！
