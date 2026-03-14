

## 一、前提条件

Claude Code：完成安装:

```
# 安装 fnm
cargo install fnm

# 添加到 PATH（cargo 默认安装到 ~/.cargo/bin）
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
echo 'eval "$(fnm env)"' >> ~/.bashrc
source ~/.bashrc

fnm install 24
fnm default 24
fnm use 24

npm install -g pnpm
```




```Shell
pnpm install @anthropic-ai/claude-code

或者走其他的方式：
pnpm install -g @anthropic-ai/claude-code --registry=https://registry.npmmirror.com
```

Playwright：安装浏览器驱动, 需确保 Claude Code 中打开的项目目录（如 ~/taos/src/TDasset）拥有正确文件权限（避免 root 所有权），执行以下命令：

```
# 在项目目录下安装 Playwright 核心库
pnpm install --save-dev playwright
# 安装 Chromium、Firefox、WebKit 浏览器驱动
pnpm playwright install

# 或许也可以使用这个
npx playwright install-deps chromium
```

可惜deepinos23 不能用，系统比较老，安装不了 chromium 等驱动

playwright-mcp-server：完成安装


```
pnpm install -g @executeautomation/playwright-mcp-server
```

```
/plugin install ralph-loop@claude-plugins-official
```





## 二、配置 Claude 使用 MCP 服务器

1. Claude 的 MCP 服务器安装完成后，配置文件默认存储在 `~/.claude.json`，无特殊要求无需修改。
2. Claude 安装后，在 `～/.claude` 目录下创建 `settings.json` 文件，文件内容如下：

```
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "sk-nEGnvyieK9nesSlsXf8rEumXm8Rzun23O2oOqCnH09LlQJvH",
    "ANTHROPIC_BASE_URL": "https://aiberm.com"
  },
  "permissions": {
    "allow": [
      "Bash(claude --version)",
      "Bash(python3:*)",
      "Bash(git *)",
      "Bash(pnpm *)",
      "Bash(npm *)",
      "Bash(node *)",
      "Bash(npx *)",
      "Bash(touch *)",
      "Bash(mkdir *)",
      "Bash(rm *)",
      "Bash(cat *)",
      "Bash(ls *)",
      "Bash(echo *)",
      "Bash(./ralph.sh)",
      "Bash(claude --version)",
      "Bash(gh *)",
      "Bash(curl *)",
      "Bash(wget *)",
      "Bash(tar *)",
      "Bash(unzip *)",
      "WebSearch",
      "WebFetch(*)",
      "Edit(*)",
      "Write(*)",
      "Read(*)"
    ],
    "deny": [
      "Bash(sudo *)",
      "Bash(rm -rf /)",
      "Bash(curl *|*sh)",
      "Bash(wget *|*sh)"
    ],
    "ask": []
  },
  "model": "opus",
  "enabledPlugins": {
    "rust-analyzer-lsp@claude-plugins-official": true,
    "ralph-loop@claude-plugins-official": true
  }
}
```



该配置文件用于定义 Claude AI 助手的运行环境、操作权限及插件启用规则，核心作用是限定 Claude 可执行的操作范围、配置运行所需环境变量，兼顾功能可用性与操作安全性，各模块说明如下：

1. **env（环境变量配置）**
   1. ANTHROPIC_AUTH_TOKEN: Claude API 访问的代理密钥 / 认证令牌，用于鉴权访问 Anthropic 服务（需替换为实际有效密钥）。
   2. ANTHROPIC_BASE_URL: Claude API 的自定义基础请求地址（如代理地址、私有部署地址），替代默认官方地址，适配内网 / 代理场景。
   3.   注：公司正在调研，调研完成后会发放token。
2. **permissions（操作权限管控）**：分为 allow（允许执行）、deny（禁止执行）、ask（需人工确认后执行）三个维度，采用「白名单优先」原则，仅 allow 内的操作可直接执行；
3. **model（Claude 模型指定）**：指定 Claude 运行的模型版本，当前配置为旗舰级的 opus 模型；
4. **enabledPlugins（启用的插件列表）**：以「插件名@插件来源」为键，布尔值为值，控制插件是否启用。



命令行开启全授权, 在 root 下不行

```
claude --dangerously-skip-permissions
```

另外一种配置：

```
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "sk-nEGnvyieK9nesSlsXf8rEumXm8Rzun23O2oOqCnH09LlQJvH",
    "ANTHROPIC_BASE_URL": "https://aiberm.com"
  },
  "permissions": {
    "allow": [
      "WebSearch",
      "WebFetch(*)",
      "Bash(*)",
      "Edit(*)",
      "Write(*)",
      "Read(*)"
    ],
    "deny": [
      "Bash(sudo *)",
      "Bash(rm -rf /)",
      "Bash(curl *|*sh)",
      "Bash(wget *|*sh)"
    ],
    "ask": []
  },
  "model": "opus",
  "enabledPlugins": {
    "rust-analyzer-lsp@claude-plugins-official": true,
    "ralph-loop@claude-plugins-official": true
  }
}
```

















## 替换为 Kimi



```
{                                                                                                                                                                                                           
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "sk-kimi-VV1biy303tlyPehBKu1e8T2HstMeizlATGfgFk7OWlQvRa0y9UyeMofR5U0QtutN",
    "ANTHROPIC_BASE_URL": "https://api.kimi.com/coding/" 
  },                                                                                                                                                                                                        
  "permissions": {                                                                                                                                                                                          
    "allow": [
      "WebSearch",
      "WebFetch(*)",
      "Bash(*)",
      "Edit(*)",
      "Write(*)",
      "Read(*)"
    ],
    "deny": [
      "Bash(rm -rf /)"
    ],
    "ask": []
  },
  "model": "opus",
  "enabledPlugins": {
    "rust-analyzer-lsp@claude-plugins-official": true,
    "ralph-loop@claude-plugins-official": true
  }
}

```









