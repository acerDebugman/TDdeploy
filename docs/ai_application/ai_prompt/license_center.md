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



culs管理 页面：

```
现在开始修复 culs管理 页面相关问题：
添加 culs 实例的流程是这样：用户会提供 customer id 和已经在 taos-culs 端init 产生的 culs-id, 所以点击 创建 CULS 弹出来的创建页面需要改为：
1. 可以填写 culs id ，customer id, 和 public key。
2. 端点输入框可以去掉，culs 连上 cls 可以产生
3. 确认提交的时候如果输入的 customer id 不存在要弹出提示
4. culs id 要保证唯一，已存在要提示
```









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





todo:

1. instance 的 resource limit 和  license 的 grant 都后期再优化，现在都用 json value 存储是合理的
2. 文档要保存更新好，因为文字阅读比代码阅读快，更抽象
3. page 分页功能可以优化，而不是将数据库表数据全部加载到内存中！
3. 







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

