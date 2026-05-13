# claude-switch

Rust CLI 工具，切换 Claude Code 的 API 连接配置。

## 项目结构

```
src/
  cli.rs          — clap 命令定义（List/Use/Add/Current/Delete/Diff）
  main.rs         — 命令分发（find_project_dir 集中调用）+ 错误处理
  lib.rs          — 模块导出
  error.rs        — CsError 枚举（exit_code/hint 方法）+ io_err/json_err
  input.rs        — 交互式输入（prompt_required/prompt_optional/prompt_confirm）
  output.rs       — 终端彩色输出（含 diff 渲染和 list 缺失状态）
  store/
    mod.rs        — validate_name + 子模块 re-export
    keys.rs       — KEY_* 常量（7 个）+ is_claude_env_key + derive_default_models
    path.rs       — 路径构造 + find_project_dir + simple_hash
    io.rs         — 文件 CRUD（profile/current/settings 读写）
    merge.rs      — merge_env_to_settings + read_current_env
  command/
    add.rs        — 交互式创建 profile
    use_profile.rs — 切换配置（merge_env + write_current）
    list.rs       — 列出 profiles + 活跃标记
    current.rs    — 显示当前 profile
    delete.rs     — 删除 profile（活跃时需确认）
    diff.rs       — 当前 env 与 profile 的文本 diff
tests/
  integration.rs  — 单元测试 + CLI 子进程测试
```

## 编码约定

- 环境变量 key 用 `KEY_*` 常量（store/keys.rs 中 7 个），不硬编码字符串
- 文件操作 TOCTOU-free：直接操作 + `match` NotFound，不先 `exists()` 再操作
- 注释只写非显而易见的 WHY，不写 WHAT
- 错误用 CsError 枚举 + exit_code/hint，不 println 后 exit
- 命令模块签名：`pub fn run(..., project: &Path) -> Result<(), CsError>`（add 除外，不需要 project）
- `colored` 仅在 output.rs 中导入，其他模块通过 output 函数使用
- 交互式输入通过 input.rs，不直接调用 stdin

## 存储

- Profile: `~/.claude-switch/profiles/<name>.json`（仅含 ANTHROPIC_* env vars）
- Current marker: `~/.claude-switch/projects/<fnv1a-hash>/current`
- Settings: 项目 `.claude/settings.local.json` 的 `env` 字段
- `CLAUDE_SWITCH_DIR` 环境变量可覆盖根目录

## use 行为

先清除 `env` 中所有 `ANTHROPIC_*` key，再写入 profile 的 key。非 ANTHROPIC_* env 和 permissions 不受影响。

## 测试

```bash
cargo test -- --test-threads=1
```

`--test-threads=1` 必须因为 `std::env::set_var` 需要单线程。测试通过 `CLAUDE_SWITCH_DIR` + tempfile 隔离，不操作真实环境。
