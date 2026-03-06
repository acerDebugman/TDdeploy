

nvm 和 nodejs 和 npm, node 这几个概念是什么？

## 逐个拆解

### 1. Node.js（平台/运行时）

- **本质**：基于 Chrome V8 引擎的 **JavaScript 运行时环境**
- **作用**：让 JavaScript 脱离浏览器在电脑上运行（服务器、脚本、工具等）
- **类比**：就像 **Android 系统**——是一个运行应用的平台

### 2. node（命令/可执行文件）

- **本质**：**Node.js 的命令行入口**

- **关系**：`node` 就是 `Node.js`，只是叫法不同

- **使用**：

  bash

  复制

  ```bash
  node app.js        # 用 Node.js 运行 app.js 文件
  node -v            # 查看 Node.js 版本
  ```

  

- **类比**：就像 `android` 这个命令（如果存在），用来启动安卓环境

### 3. npm（Node Package Manager）

- **本质**：**Node.js 的默认包管理工具**
- **关系**：随 Node.js **捆绑安装**（装 Node 时自动获得 npm）
- **作用**：下载、管理第三方代码包（如 React、Express、Lodash）
- **类比**：就像 Android 系统自带的 **Google Play 商店**
- **注意**：虽然绑定发行，但 npm 有自己独立的版本号（如 Node v20 可能带 npm v10）

### 4. nvm（Node Version Manager）

- **本质**：**第三方版本管理工具**（非 Node.js 官方组件，但社区标准）
- **作用**：在一台电脑上安装、切换**多个版本**的 Node.js
- **为什么需要**：
  - 项目 A 需要 Node 16，项目 B 需要 Node 20
  - 测试代码在不同 Node 版本的兼容性
- **类比**：就像 **多系统启动管理器**（如 Grub），让你在 Windows 10 和 Windows 11 之间切换

```
┌─────────────────────────────────────┐
│           你的电脑                   │
│  ┌─────────────────────────────┐    │
│  │         nvm（管家）          │    │
│  │  ┌─────────────────────┐    │    │
│  │  │   Node.js v20       │    │    │
│  │  │  ┌───────────────┐  │    │    │
│  │  │  │  node（命令）  │  │    │    │
│  │  │  │  npm（包管理） │  │    │    │
│  │  │  │  + 自带模块    │  │    │    │
│  │  │  └───────────────┘  │    │    │
│  │  └─────────────────────┘    │    │
│  │  ┌─────────────────────┐    │    │
│  │  │   Node.js v18       │    │    │
│  │  │  ┌───────────────┐  │    │    │
│  │  │  │  node + npm   │  │    │    │
│  │  │  └───────────────┘  │    │    │
│  │  └─────────────────────┘    │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
```



## pnpm 安装

是 npm 的优化版本

```bash
npm install -g pnpm
```





playwright 安装:

```bash
pnpm add -D @playwright/test --ignore-workspace-root-check

pnpm exec playwright install

pnpm exec playwright test
```

