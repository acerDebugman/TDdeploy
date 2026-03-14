## 开发

### 初始化阶段：

```
你是一个精通 rust 和 vue 的开发架构师，具有丰富的开发经验。
文档 kimi/License_Center-FS.pdf 是描述这个项目的功能文档，里面涉及到


你是一位拥有10年经验的资深全栈架构师，精通 Rust 系统编程和 Vue3 企业级前端架构。你的任务是根据文档 kimi/License_Center-FS.pdf 描述功能，生成一个完整的、生产级的全栈项目框架。

## 第一阶段：需求解析与架构蓝图（分析模式）

首先，深度解析用户提供的功能说明文档，提取以下要素：
- 核心业务领域（DDD 限界上下文划分）
- 用户角色与权限体系（RBAC 模型）
- 关键业务实体与流程（状态机、生命周期）
- 非功能性需求（QPS、并发、数据一致性、延迟要求）
- 第三方集成点（支付、消息推送、文件存储等）

基于分析，生成《架构决策记录》(ADR)：
1. 技术选型理由（为什么选 Axum/Actix、Sea-orm/Diesel、PostgreSQL/MongoDB）
2. 架构分层策略（清晰的分层/六边形架构/CQRS 是否适用）
3. 部署拓扑（单体/微服务/Serverless）
4. 安全策略（JWT/OAuth2、数据加密、审计日志）

## 第二阶段：项目脚手架生成（构建模式）
生成完整的项目文件树（使用特定标记便于 CLI 解析）：

### 后端架构（Rust）
生成完整可编译的 Cargo workspace 结构。
**必须包含的代码规范：**
- 所有领域实体使用 Newtype 模式（如 `pub struct UserId(Uuid)`）附带 `Deref` 实现
- 错误处理：定义 `AppError` 枚举实现 `IntoResponse`，禁止裸 unwrap
- 异步边界：明确区分 IO 密集型（async）和 CPU 密集型（spawn_blocking）
- 配置管理：使用 `config` crate 实现结构化配置（数据库连接池、JWT 密钥等）
- 健康检查：包含 `/health` 和 `/ready` 端点，带数据库连通性检查

### 前端架构（Vue3 + TS）
生成基于标准 Vite 的项目, 使用 element-ui 完成前端设计和开发。
**必须包含的代码规范：**
- 所有 API 调用返回类型必须定义，使用 `zod` 进行运行时校验（防御性编程）
- Props 定义使用 `type Props = {...}` + `defineProps<Props>()`，禁止 `any` 类型
- 组件导出使用 `defineExpose` 显式声明公共方法
- 状态管理：Store 中处理所有异步逻辑，组件仅调用 actions
- 错误处理：全局错误边界捕获未处理异常，API 错误分级（通知/跳转/重试）

## 第三阶段：核心领域代码生成（实现模式）

基于功能说明文档，生成关键业务逻辑的骨架代码（可编译通过）：

## 第四阶段：质量基础设施（管控模式)
需要通过 rust 的编译和前端编译
fmt + clippy（严格模式）

## 第五阶段：交付物规范（输出格式）
你的输出必须严格按照以下格式，输出到 docs 文档里，方便查阅。

project_structure:
  - path: "relative/path/to/file"
    type: "file|directory"
    description: "文件用途说明"
    content: |
      # 如果是文件，这里放完整代码内容
      # 使用正确的缩进保持 YAML 格式
      [代码内容]
      
  - path: "README.md"
    type: "file"
    description: "项目启动指南"
    content: |
      # 项目名称
      
      ## 快速开始
      1. 安装依赖：`cargo install sea-orm-cli` 和 `pnpm install`
      2. 环境配置：`cp .env.example .env` 并填写数据库配置
      3. 启动服务：`make dev`
      
      ## 架构说明
      [根据功能文档生成的架构说明]

setup_commands:
  - "cd backend && cargo build"
  - "cd web && pnpm install"
  - "docker-compose -f docker-compose.dev.yml up -d"

quality_checkpoints:
  - "后端编译零警告（cargo build 无 warnings）"
  - "前端 TypeScript 严格模式检查通过"
  - "所有生成的接口都有对应的测试桩"
  - "数据库迁移文件包含回滚逻辑"

```





### 需求：

1. 4 个命令，2个 sdk: java 和 c 
1. 版本升级问题，



### 迭代功能

1. 离线 时，需要添加 离线 原因 这一列：因为可能出现 客户端不对，导致离线 或者 culs 确实没有启动上线
2. 非离线状态， reason 就为空
3. 





### 迭代开发阶段

1. 测试环境准备，让 ai 自动链接



#### 完善 culs

```
当前项目未完成状态，继续开发完善。
架构上的描述同前述的文档，现在再补充一些信息：
1. cls 部署在公司服务器上，culs 需要部署在客户侧，用于 tsdb/idmp 授权
2. 此项目还需要 c/java 写的 sdb 提供给 tsdb/idmp 使用，这两个 sdb 的作用是通过 culs 服务的接口，获取授权维度的信息
3. 当前项目 cls 的 web服务启动命令改为 taos-cls，命令行工具改为 taos-cls-admin，当前的 web 项目建议改名为 cls-web 或者 cls_web。
4. 目前还缺少 culs 的后端实现和 web 页面实现，culs 的服务启动命令改为 taos-culs，对应的命令行工具改为 taos-culs-admin。culs 的 web 项目建议取名为 culs-web 或者 culs_web。
5. 当前还缺少 c/java 的 sdk 实现

根据上面补充的架构信息，继续完善 docs 里的文档。

开发上要注意：
1. cls 项目使用 pg 数据库做存储
2. culs 部署客户侧，需要使用 RocksDB 作为存储

cls 使用的 pg 数据库的链接信息已经在配置文件：config/development.toml 里。

开发完成后，你需要启动服务并且检查 cls 服务的表是否已经在 pg 数据库建立成功。

开发完成后需要验证的测试用例：
1. 编写 cls 服务的测试用例，测试 cls 的 api 接口和命令行工具
2. 编写 culs 服务的测试用例，测试 cls 的 api 接口和命令行工具
3. 编写 sdk 库的测试用例，用于验证 sdk 的功能是否完整

```



```
 cls 服务的代码文档整理：
 1. 将当前 cls 服务的表和字段，含义相关信息写成文档放到 docs 目录里
 2. 将当前 cls 服务实现的所有 api 接口写成文档放到 docs 目录里
```



```
当前执行 cargo run --bin taos-cls 后，在 pg 数据库里并没有任何的表生成，修复这个问题，并使用接口进行测试
```



1. 升级 cargo edition 2024
2. cls-web 升级用 vue3 + element-plus 版本
3. culs-web 项目开发 



```
将 cls-web 用 vue3 + element-plus 从新改写
```



小迭代：

1. 增加 update_at 更新时间：

   ```
   如果 customer 实体发生了更新，则需要增加更新修改时间，给我在相关的位置增加表示最近一次更新时间的字段。并运行 taos-cls 进行表重建。
   ```

2. ```
   将 Culs 实体的 status 字段 CulsStatus 改为只有3种状态, online, offline, blacklisted
   ```

3. ```
   完善api接口 /api/v1/customers 的 POST 和 GET 方法，并在 tests 里编写测试用例，测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过。
   ```

4. ```
   完善api接口 /api/v1/customers/:customer_id 的 delete 方法，并在 tests 里编写测试用例，测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过。
   ```

5. ```
   完善 culs 相关操作的 api：
   1. 支持创建 culs, status 默认是 offline
   2. 支持删除 culs
   3. 支持更新 culs，但是只支持更新 status 字段
   ```

6. ```
   在 tests 里编写 culs api 相关的测试用例，做法类似 customer 的测试用例。测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过，并且清理输出。
   ```

7. ```
   完善 license 相关操作的 api, 并在 tests 里编写 culs api 相关的测试用例，做法类似 customer 的测试用例。测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过，并且清理输出。
   ```

8. ```
   完善routes.rs 里现有的 blacklist 相关的 api 功能, 并在 tests 里编写 blacklist api 相关的测试用例，做法类似 customer 的测试用例。测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过，并且清理输出。
   ```

9. ```
   实现 instance 的 查看 instance 的 api 功能, 并在 tests 里编写 instance api 相关的测试用例，做法类似 customer 的测试用例。测试用例使用 config/development.toml 里的 url 的pg数据库配置 ，先保证编译可以通过，然后启动 taos-cls，保证测试用例可以通过，并且清理输出。
   ```

10. ```
    ```

11. 



/init 先使用这个命令将项目总结了一下，产生了 Agent.md 总结文档，

以及产生的信息：

```
Key Development Commands

  # Development
  docker-compose -f docker-compose.dev.yml up -d  # Start PostgreSQL
  cargo run --bin taos-cls                         # Run CLS
  cd cls-web && npm run dev                       # Run frontend

  # Testing
  cargo test                                      # Rust tests
  npm run test:unit                               # Frontend tests

  # Quality
  cargo clippy -- -D warnings                     # Lint Rust
  cargo fmt                                       # Format Rust
  npm run lint                                    # Lint frontend

```







cls-web 页面迭代：

客户管理页面：

```
cls-web 项目是 cls 的 web 页面，现在 http://172.17.0.2:8080 是后端 cls 项目的访问地址，cls-web 的访问地址是http://172.17.0.2:3000。
现在先完成以下功能：
1. 添加 culs 管理页面
2. 添加 instance 的管理页面
3. 完善 customer 客户管理的相关功能，当前数据有数据，也无法显示在页面上

注意事项：
1. 后端 api 接口统一使用 snake_case 规范，并将这个规范更新到 docs/cls-api-documentation.md 中去
```

```
cls-web 项目是 cls 的 web 页面, 现在完成以下功能：
1. 添加 culs 管理页面
2. 添加 instance 的管理页面
```

```
修复 客户管理 相关的问题：
1. 页面上每行数据要显示客户id
2. 完成客户管理页面的搜索功能
3. cls 后端增加对 name 的唯一约束

修改完成后，增加对应的 cls 测试用例和 cls-web 的客户管理的操作相关的测试用例
```

```
继续修正 客户管理 问题:
1. 取消点击数据时就进入客户详情页，只有点击查看的时候才能进入客户详情页
2. 编辑或者创建的时候，customer 名称冲突时的提示词修改为: 名称已存在
```

```
继续修正 客户管理 相关问题:
1. 分页功能有bug,现在里面有22条数据，分页功能无法正确分页显示

现在 客户管理 分页依然有问题:
1. 现在问题是当期页面有数据27条，但是无论怎么选择分页大小，页面依然一直显示27条
```

```
总结一下前面所有修复的 客户管理 页面的问题经验.

ai:
层面         教训
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   数据库       唯一约束应在数据库层强制，应用层做二次校验
   后端 API     total 应返回总记录数，不是当前页数量；分页需在服务端做数据切片
   ORM 使用     确认方法存在性，避免使用不存在的方法（如 Sea-ORM 没有 contains）
   前端响应式   Pinia store 的嵌套 ref 属性不能直接用 v-model 绑定，需用 computed 包装
   测试设计     使用互斥数据 + 严格断言 + 负面断言（验证未匹配数据不存在）
   部署         后端修改后必须重新编译并重启服务才能生效

```



#### culs管理 页面

```
现在开始修复 culs管理 页面相关问题：
添加 culs 实例的流程是这样：用户会提供 customer id 和已经在 taos-culs 端init 产生的 culs-id, 所以点击 创建 CULS 弹出来的创建页面需要改为：
1. 可以填写 culs id ，customer id, 和 public key。
2. 端点输入框可以去掉，culs 连上 cls 可以产生
3. 确认提交的时候如果输入的 customer id 不存在要弹出提示
4. culs id 要保证唯一，已存在要提示
```

```
当前创建 culs 页面提交后，culs id 还是自动生成的，会覆盖用户输入的 culs id, 这里改为使用用户输入的 culs id, 后端做合法性校验后再入库
```

```
修正 culs管理 页面相关问题:
分页功能有bug, 现在里面有 17 条数据，分页功能无法正确分页显示，选择 10/page 的时候，页面依然还是显示所有的 17 条数据 
```

```
修正 culs管理 页面 操作列 的相关问题:
1. 去掉操作列的 注册 这个功能
2. 编辑功能页面添加 客户端ID，支持重新选择 客户端ID
3. 编辑功能页面 状态 选择变为只能选择 黑名单 或者 留空白不选，不选就是不改变当前状态
```

```
修正 culs管理 页面 删除 操作的相关问题: 确认删除某条记录后，数据页面没有刷新。比如总的 21 条数据，当前页面每页展示20条，总共2页，删除一条数据后。总的显示20条，变为一页，但是当前这一页只有 19 条。
修复这个问题
```

```
culs管理 页面的搜索部分增加一个按 culs id 搜索的下拉框，查询条件支持按 culs id 查询某条数据
```

```
culs管理 页面的搜索部分 客户 这个搜索的下拉框，目前看似乎是按客户名检索，修改为按 客户id 检索，并且标签名改为 客户ID
```



#### 许可证管理页面

```
修正 签发许可证 页面的相关问题:
1. 客户 这个下拉选择后出现 资源不存在 的 alert 错误
2. 客户 这个下拉框的标签名改为 客户名
3. 客户 这个下拉框选择后，增加一列显示出 客户ID
```



```
修正 签发许可证 页面的相关问题:
1. 当前使用 客户名 PaginationTest41370bba93b04acf885100efe835da6aCustomer4 选择下拉框后，culs 下拉框没有可选项，当前数据库是存在 PaginationTest41370bba93b04acf885100efe835da6aCustomer4 关联的 culs 的
```



```
修复 许可证管理 页面的相关问题:
1. 数据列表的 客户，类型，创建时间 都没有正确显示
2. 确认 删除 操作后，出现 请求参数错误: 请求参数错误 的弹窗错误提示
```

```
修复 许可证管理 页面的相关问题:
1. 取消 取消整行点击事件 跳转许可证详情页，只有点击查看的时候，才可以进入到许可证详情页
2. 添加 culs id 列
3. 完善 搜索 功能，添加支持：按 culs id 搜索，客户名搜索
```



```
修复 许可证管理 页面的相关问题:

请求参数错误: 请求参数错误
```



```
许可证管理 的操作列，添加 吊销 操作，点击后实现功能是：
1. 将此条 license 记录改为 吊销 状态
2. 将此条 license 记录加入到黑名单
```



```
签发许可证页面在确认签发的时候，功能上添加一个约束：每个 culs 只能有一个 created 状态 或者 active 状态的 license, 需要先revoke这个 license 才可以创建， 其他 license 状态可以正常创建。
如果约束不满足需要弹窗提示
```



claude:

```
实现这个功能: 
1. 签发许可证页面在确认签发后，cls 需要将这个 license 通过 libp2p 推送给 culs，并且　cls 发送给 culs 的 license 做签名身份验证, cls 使用固定的 ed25519 的秘钥 QJn+kkNwSvdCgUJik8OnHQaJpxej7AWXERcfFQcWPR8=， culs 使用固定公钥：vRTTWNBW6Y1V528apbqDiFibPJSggeIOSaks3aabGJM= 
2. culs 收到 license 后，保存起来，让 culs-web 的 许可证管理页面 可以展示
3. 修复 culs-web 的 许可证管理页面 页面相关的展示 bug, 展示方式参考 cls-web 的 许可证管理页面 页面逻辑
```



```
cls 需要处理一下 culs 发送回来的 SyncResponse::Error 消息
```

```
将 mpsc::Receiver 换为 flume 组件的异步方式收发消息
```

```
修复：cls/tests 里的测试用例问题
```

```
现在 culs 函数 verify_license_signature 验证 cls 发送 license 时出错:
经过我的排查 cls 中现在 signature 应该并没有生成，且签名时和验证时使用的 license_data 不一致，并修复相关问题
```

```
先对齐一下 cls 里的 LicenseGrants 和 license-p2p-protocol 里的 LicenseGrants：使 license-p2p-protocol 的 LicenseGrants 和 cls 里的 LicenseGrants 保持一致
```

```
license_handler.rs 里也要有对应的 signature 和 license_data 的生成逻辑，offline token 的生成逻辑：将 license_data base 64 加密后 用 "." 符号 链接 signature, 形式是：${license_data}.${signature}, 并保存到数据库中；
culs 离线状态收到 offline token 后，也需要这样反解，并验证 token；

cls 的 offline_token 的 token 在 license_handler.rs 也需要一起产生保存；同理命令行的 issue license 也要保存 offline_token
```



```
将 cls 的接口 /licenses/offline 合并到 /licenses 里，创建 license 的时候，同时 /licenses 也产生 offline_token. 
cls-web 也同步修改，确认签发的时候，只需要调用一个接口就行
```



```
给 culs-web 项目的 许可证管理 页面添加 吊销 和 删除 操作：
1. 删除操作直接删除数据记录即可，但是 active 状态的 license 数据不可以删除
2. 吊销操作则将 license 放入到 黑名单，且发 吊销 license 的消息给 cls
3. cls 接收到 culs 的 license 的吊销消息后，执行已有的 revoke license 的流程



我需要完成一个复杂任务：
给 culs-web 项目的 许可证管理 页面添加 吊销 和 删除 操作：
1. 删除操作直接删除数据记录即可，但是 active 状态的 license 数据不可以删除
2. 吊销操作则将 license 放入到 黑名单，且发 吊销 license 的消息给 cls
3. cls 接收到 culs 的 license 的吊销消息后，执行已有的 revoke license 的流程

请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```



```
culs-web 页面的 CULS节点管理 页面新增一个 当前服务地址 的数据项，表示当前 culs 服务提供的 libp2p 连接地址，使用 quic 服务的形式，注意: culs 后端应该把这个地址保存到 CulsIdentity 里，方便展示出来
```















```
cls 侧 offline token 产生逻辑：将 grants 序列化为json字符串，然后使用 ed25519 的 private key 进行签名，
```









#### 黑名单管理

```
1. 实体


```



dashboard 页面：

```
仪表盘 页面里的 即将过期的许可证 表格的数据展示功能不全： 客户，类型，过期时间 都未展示出来，修复这个问题
```

```
在添加了 culs, 客户管理，以及黑名单操作 后，仪表盘 的 最近活动 表格没有展示出数据，修复这个问题，如果还有其他关键操作也没有加入审计日志，也一同修复
```



```
```



#### 通信模块：

```
完成功能：cls 与 culs 是通过 libp2p 进行通信， cls 要实现 p2p 的 ping, bootstrap, kad, request-response 协议功能，并且 culs 每隔 30s 向 cls 发送心跳信息，cls 需要更新  culs 状态。互相连通后，
1. cls 会发送已经授权的 license 给 culs, culs 需要将 license 入 rocksdb 库
2. culs 侧会将本地注册的实例信息发送给cls, 并入 cls 的库。
```



```
完成功能：
1. cls 与 culs 在启动的时候，都需要启动 libp2p 服务, 并且使用 0.0.0.0 的网卡地址
2. 每个 culs 在 libp2p 启动后，需要主动去连接 cls, 使用 init 初始化给定的 cls 地址去连接，这个 tcp 地址注意要转为 libp2p 的 multiaddr 地址
4. 在 culs 的 identity 的管理字段里，应该有个 status 的状态，这个状态与 cls 里的 culs 表字段 status 的含义一致。如果 culs 如果没有链接 cls，这个 status 字段表示 offline; 如果 culs 启动后连接上了 cls， 应该更新为 online 表示在线状态；如果接收到 cls 将 culs 拉入黑名单的消息，应该更新为 blacklisted
```



```
将 culs main.rs 里的 libp2p 的启动部分包装或者抽象为类似 http 服务的启动的方式; 注意将业务逻辑移到应用层，处理返回的 event_rx, 并且添加如果 p2p 服务没有启动成功，就显示错误原因，并且整个程序退出
```



```
将 culs 和 cls 的 libp2p 通信的 request-response 协议共同使用的结构体抽取出来作为一个项目，通过 cargo.toml 里依赖的方式添加到 culs 和 cls 项目中，方便 libp2p 通信
```



```
我想要cls 的 peer id 使用 deterministic generation 方式产生，即给一个种子数字，然后产生固定 peer id, 这样不依赖物理机器，可以更换机器，也可以让各地部署的 culs 方便链接，实现这个功能
```



```
当前 cls 和 culs 在互发心跳过程中，如果 cls 重启后，culs 会断开了链接，并且不会再重新发送心跳，修复这个问题
```









cls 的身份验证：

```
public key: vRTTWNBW6Y1V528apbqDiFibPJSggeIOSaks3aabGJM=
private key: QJn+kkNwSvdCgUJik8OnHQaJpxej7AWXERcfFQcWPR8=
```



culs 开发：

```
taos-culs 需要支持命令行模式，支持这个 init 命令，init 命令使用方式：
taos-culs init \
  --central-server "license.taosdata.com:8443" \
  --customer-id "customer-001" \
  --data-dir /var/lib/taos/culs
  --force
参数含义：
--central-server 表示 cls 服务地址
--customer-id 表示客户ID
--data-dir 表示数据目录，可选参数，未指定则使用默认数据目录 /var/lib/taos/culs
--force 表示强制清理掉旧的数据并重新产生新的数据
输出：
CULS ID 和公钥，此公钥用于与 cls 通信使用, culs id 用于在 cls 中创建 culs 实例。

内部实现的一些功能逻辑：
1. 产生的 CULS ID 和 公钥 需要保存到 rocksdb 中，一个数据目录只能表示一个 culs，即只有一个 culs id
2. 增加一个类似 culs 的管理页面，展示当前 culs 节点的具体信息：
  a. 没有 init 的状态，如果 culs 没有 init 就启动，显示 culs 未初始化 字样, 并且提供操作按钮，可以实现 init 功能，但是只能指定 cls 服务地址 和 客户ID 这两个参数, 初始化后
  b. 如果已经初始化过，即 rocksdb 中已经存在 culs id， 就显示对应的 culs id，公钥信息
3. 如果是未初始化状态，所有写入接口，都应该返回提示 culs未初始化 信息

注意：
1. culs 的 init 功能的 http 接口 与 命令行 接口的实现应该使用同一层抽象
```

```
继续在 culs-web 上增加一个类似 culs 的管理页面，展示当前 culs 节点的具体信息：
  a. 没有 init 的状态，如果 culs 没有 init 就启动，显示 culs 未初始化 字样, 并且提供操作按钮，可以实现 init 功能，但是只能指定 cls 服务地址 和 客户ID 这两个参数, 初始化后展示 culs id 和 公钥信息
  b. 如果已经初始化过，即 rocksdb 中已经存在 culs id， 就显示对应的 culs id，公钥信息
```





culs-web开发：

```
我现在模仿 cls-web 创建了 culs-web 项目，culs-web 主要功能是实现 culs 自己拥有的 许可证管理，实例管理，黑名单管理，再提供类似 cls-web 的 dashboard 页面。现在 npm run dev --host 后，登录 /login 页面会报错，修复这些问题，让 npm run dev --host 的 登录功能可以正常运行。
```

```
继续开发 culs, 支持 culs-web 所需要的接口。culs 的存储是使用 rocksdb 进行存储，这是运行在客户侧的服务，主要实现 许可证管理，实例管理，黑名单管理，许可证管理是对当前 culs 所拥有的 license 进行管理。
```



测试数据：

```
在 tests 目录里给我用 rust 写一个查看 culs 的 data 的工具，项目名叫 taos-culs-data, 这个工具可以查看 culs 里指定 culs id 的数据。
使用方式：
taos-culs-data -d ./culs-data -i culs-2e26a5ee-115c-427f-af68-f311687169fa -t license
参数解释：
-d 表示 culs 的 rocks db 数据目录
-i 表示 culs 的 id
-t 表示表 license

由于 culs 使用的是 rocksdb 保存的数据，读取的数据格式需要与 culs 的写入的数据格式保持一致
```







#### SDK管理

```
设计一个 rust 的 sdk, 实现以下设计：
1. 使用 libp2p 链接 culs 实例
2. 链接上 culs 实例后，自动获取当前正在使用的 license
3. sdk 里使用 culs 的公钥，去验证 license 的合法性
4. 

用 rust 在 tests 目录里新建一个 test-instance 的项目，这个项目启动后使用 rust 的 sdk 获取 license 授权信息，并打印出来；
```



```
我需要完成一个复杂任务：
新增设计一个 rust 的 sdk, 实现以下设计：
1. sdk至少包含两个函数：一个是通过 libp2p 链接 culs 实例 获取 license 的函数，一个是 loop 发送心跳的函数
2. 函数接口必须包含参数：libp2p 链接 culs 实例地址
3. 链接上 culs 实例后，自动获取当前正在使用的 license，状态不能是 revoked 或者 过期的 的
4. sdk 里使用 culs 的公钥，去验证 license 数据的合法性
新增一个测试实例项目，用rust语言编写，要求如下：
1. 测试项目叫 test-instance, 在 tests 目录下
2. 使用 rust 的 sdk 去获取 license 并且打印出来，然后发送心跳
3. 这个项目就是为了开多个实例节点去测试 sdk 的功能，验证culs对实例的管理功能，你可以自由发挥

请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```



```
我需要完成一个复杂任务：
rust SDK 与 culs 通信获取 license 的接口似乎没有发送消息给 culs, 设计实现这块与culs通信的协议，并且注意与 culs 和 cls 的通信协议区分。

请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```



```
我需要完成一个复杂任务：
instance 链接 culs 获取 created 的 license 后，license 状态改为 active 的，并且发送 update license 的消息给 cls, cls 里对应的 license 状态也同步修改为 active

请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```



```
我需要完成一个复杂任务：


请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```







license 已吊销，就是等于黑名单了！
























## 开场让模型记住的提示词

这部分提示词应该是开发中的经验总结，然后让模型记住，避免后续遗忘。

测试用例提示词：

```
关键教训
1. 不要只用数量验证：要用返回数据的内容验证
2. 使用互斥的测试数据：确保搜索结果可以明确区分是否过滤正确
3. 使用严格的等于断言：不要用 >= 或 <=
4. 验证反向条件：验证不匹配的确实没有返回
```

经验教训提示词：

```
   数据库       唯一约束应在数据库层强制，应用层做二次校验
   后端 API     total 应返回总记录数，不是当前页数量；分页需在服务端做数据切片
   ORM 使用     确认方法存在性，避免使用不存在的方法（如 Sea-ORM 没有 contains）
   前端响应式   Pinia store 的嵌套 ref 属性不能直接用 v-model 绑定，需用 computed 包装
   测试设计     使用互斥数据 + 严格断言 + 负面断言（验证未匹配数据不存在）
   部署         后端修改后必须重新编译并重启服务才能生效
```















文档更新：

```
根据现在的代码将 docs/cls-api-documentation.md 和 docs/cls-database-schema.md 的内容更新
```





### todo:

1. instance 的 resource limit 和  license 的 grant 都后期再优化，现在都用 json value 存储是合理的
2. 文档要保存更新好，因为文字阅读比代码阅读快，更抽象
3. page 分页功能可以优化，而不是将数据库表数据全部加载到内存中

#### 20260227

1. 剩下 仪表盘 和 黑名单管理 两个页面 (done)

#### 20260302:

1. 修改每个管理页面的 1，2 。。。 的数字列
2. license 需要进行 update 操作，dashboard 页面的 续签 要跳转到 license 更新页面，并且 参数 写死










### bug

1. 当前 登录页面 如果后端没有启动，则报错: 资源不存在 , 改为对应后端未启动等
2. culs 测试 重启后，instance 需要能够连接上，这在 heartbeat 需要用；或者只在启动的时候使用
2. cls 下发 license 后，instance 是否需要实时获得
2. culs  与  cls 的 license 的数据校准
2. 1m 自动检测 license 过期的服务，并且cls/culs 互发，两个随机 30-1m 之间吧
2. 初始化自动同步 license 和 黑名单 数据，并进行一致性同步，通过 get license 和 get blacklist 命令附带数据进行







## 总结输出：

```
 完成的 Blacklist API

  端点

   方法     端点                      功能
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   GET      /api/v1/blacklist         列出所有黑名单条目
   POST     /api/v1/blacklist         添加到黑名单
   DELETE   /api/v1/blacklist/:id     从黑名单移除
   GET      /api/v1/blacklist/check   检查实体是否在黑名单

  文件修改

  1. 实体模型 - entities/blacklist.rs - Sea-ORM 实体
  2. Repository - repositories/blacklist_repo.rs - 数据库操作
  3. DTO - dtos/blacklist_dto.rs - 请求/响应数据结构
  4. Handler - handlers/blacklist_handler.rs - API 实现

  测试用例（8个全部通过）

   测试                                        描述
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   test_add_to_blacklist_success               成功添加到黑名单
   test_add_to_blacklist_validation_error      验证空 entity_id 失败
   test_add_to_blacklist_invalid_entity_type   验证无效实体类型失败
   test_list_blacklist                         获取黑名单列表
   test_check_blacklist                        检查实体黑名单状态
   test_remove_from_blacklist                  从黑名单移除
   test_remove_from_blacklist_not_found        移除不存在的条目返回 404
   test_blacklist_update_existing              更新已存在的黑名单条目

  运行测试

  cd /app/license_center/cls
  cargo test --test blacklist_api_test

```









## 开发经验

1. 梳理清楚执行流程，可以按使用场景梳理，否则很难构建整个系统的状态

2. 系统的状态是根据场景的执行流程或者说数据流转产生的，所有执行流程很重要

3. 执行流程应该按场景分类，都应该是有限场景！

4. api, schema 变更的文档要保存更新好，因为文字阅读比代码阅读快，更抽象

5. 每一步都要git

6. 测试用例很重要，每次都要有测试用例保存！

7. 先形成 mvp 最小的版本，前后端打通，然后再进行迭代，这样可控性更强

8. 每次开发完一个功能，可以让模型总结开发经验，避免模型再犯同样的错误，比如：总结一下前面所有修复的 客户管理 页面的问题经验.

   然后将这些经验放到开场提示词中。



每一步做什么：

1. 描述需求，审阅代码，测试用例，git提交





人应该做什么：

1. 扩展知识边界，熟悉新的库，熟悉理论，熟悉领域知识，学习新代码库的使用
2. 将熟悉后的知识，转为代码让 agent 执行！







## 感受

1. 总体来说：非常好，效率很高，并且很享受开发的感觉
2. 经验丰富的工程师非常有用，因为模型为了快速达成目标，使用的方法往往不够优化或者有坑，经验丰富的工程师可以一眼看出
3. 



## 测试



```
 验证结果

  # TypeScript 严格检查通过
  npx vue-tsc --noEmit
  # ✅ 无错误

  # 生产构建成功
  npm run build
  # ✅ 构建完成

  # 开发服务器正常启动
  npm run dev
  # ✅ http://localhost:3000/

  现在可以正常访问：

  • 开发环境：http://localhost:3000
  • 登录页面（默认账号密码任意输入即可）
  
  
    npm run dev -- --host 0.0.0.0
  # 或
  npx vite --host 0.0.0.0 --port 3000

```



pg 数据库：

```
psql -U license_center -w license_center


```



## 产品如何设计

1. 控制入口
2. 保持简洁和一致
3. 



## 使用 claude code 完成 culs 信息开发和 sdk 开发









## cc使用感受

1. 模型层面没感觉比 kimi 强，跨项目的开发，需要依然人协调，一个项目里开发实现一致性较好，可能和 context window 大小有关系，连续性没有那么好
2. cc 比 kimi-cli 强一些

3. 





### 复杂任务

```
我需要完成一个复杂任务：[具体描述]

请按以下格式处理：
1. 先分析任务并生成详细的 Todo List（用 - [ ] 标记）
2. 每完成一步，勾选 - [x] 并简要说明
3. 遇到阻塞或需要我确认时暂停，等待我的输入
4. 最后生成总结报告

开始吧。
```



执行完：效果还可以，虽然有点小瑕疵!!



在 CLAUDE.md 里配置 复杂任务处理协议：

```
# 复杂任务处理协议

当用户提出包含多个步骤的复杂任务时：

1. **规划阶段**
   - 生成带编号的 Todo List
   - 标注优先级（P0/P1/P2）
   - 估算每步耗时
   - 询问用户："这个计划是否可行？需要调整吗？"

2. **执行阶段**
   - 每完成一项，在代码块中更新进度：
     ```progress
     [3/7] ✅ 完成数据库配置
         ⏳ 当前：编写 API 路由
         ⏭️  下一步：添加验证中间件
     ```
   - 关键节点要求确认："已完成 X，是否继续 Y？"

3. **检查点机制**
   - 每 3-5 步自动创建 git commit
   - 使用 `git checkpoint: [步骤描述]`
```





## skill

1. 构建启动 skills 
2. 





## mcp

1. pg 的 mcp
2. 







