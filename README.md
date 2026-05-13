# claude-switch

CLI 工具，切换 Claude Code 的 API 连接配置。

在不同项目间快速切换 `ANTHROPIC_BASE_URL`、`ANTHROPIC_API_KEY` 等环境变量，无需手动编辑 `.claude/settings.local.json`。

## 安装

```bash
cargo install claude-switch
```

或从源码构建：

```bash
git clone https://github.com/jsx3323/claude-switch.git
cd claude-switch
cargo build --release
```

## 使用

### 添加配置

```bash
claude-switch add work
```

交互式输入 `ANTHROPIC_BASE_URL`、`ANTHROPIC_API_KEY`、`ANTHROPIC_MODEL`，可选自定义四个子模型 key。

### 切换配置

```bash
claude-switch use work
```

写入项目的 `.claude/settings.local.json` `env` 字段，仅修改 `ANTHROPIC_*` 相关变量，不影响 `permissions` 和其他非 ANTHROPIC key。

互斥认证变量自动处理：切换含 `ANTHROPIC_API_KEY` 的配置时，已有的 `ANTHROPIC_AUTH_TOKEN` 会被清除；反之亦然。

### 认证方式：API_KEY 与 AUTH_TOKEN

Claude Code 支持两种认证方式，它们是互斥的：

- `ANTHROPIC_API_KEY` — 直接用 API Key 认证
- `ANTHROPIC_AUTH_TOKEN` — 用 OAuth Token 认证（如 Claude Code 内置登录生成的 token）

**冲突处理规则**：`use` 命令不会删除所有 `ANTHROPIC_*` 变量，只删除与当前配置冲突的 key。如果 profile 包含 `ANTHROPIC_API_KEY`，则 settings 中已有的 `ANTHROPIC_AUTH_TOKEN` 被清除；如果 profile 包含 `ANTHROPIC_AUTH_TOKEN`，则已有的 `ANTHROPIC_API_KEY` 被清除。其他 `ANTHROPIC_*` 变量（如 `MODEL`、`BASE_URL`）不受影响。

**常见场景**：你通过 Claude Code 内置登录（`claude login`）生成了 `AUTH_TOKEN`，之后想切换到一个自定义 API 代理（需要 `API_KEY`）。`claude-switch use` 会自动清除 `AUTH_TOKEN`，避免两种认证方式同时存在导致冲突。

### 查看当前配置

```bash
claude-switch current
```

### 列出所有配置

```bash
claude-switch list
```

### 查看差异

```bash
claude-switch diff staging
```

对比当前环境变量与指定配置的文本 diff。

### 删除配置

```bash
claude-switch delete old-config --force
```

删除活跃配置时默认提示确认，`--force` 跳过。

## 存储

| 数据 | 位置 |
|---|---|
| Profile | `~/.claude-switch/profiles/<name>.json` |
| 当前标记 | `~/.claude-switch/projects/<hash>/current` |
| 项目配置 | `<project>/.claude/settings.local.json` 的 `env` 字段 |

`CLAUDE_SWITCH_DIR` 环境变量可覆盖根目录。

## 命令别名

| 命令 | 别名 |
|---|---|
| `list` | `ls` |
| `current` | `show` |
| `delete` | `rm` |

## License

MIT