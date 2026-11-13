# AGENTS.md — {{ project-name }}

{{ description }}

> 本文件是给 AI 编码代理的项目说明书。**外围基础设施已由模板生成，不要改动它们**，只在 `src/` 里写业务逻辑。

## 命令（统一用 `just`，不要直接调 cargo）

```bash
just check        # 类型检查
just build        # debug 构建（SHELL_HELP_DIR=shell_help，顺带生成补全和 man）
just release      # release 构建
just test         # 测试
just fmt          # 格式化
just fmt-check    # 格式检查（不修改）
just lint         # clippy
just lint-strict  # clippy -D warnings（CI 用这个）
just ci           # 全量门禁：fmt-check → lint-strict → test → release
just install      # 安装到 /usr/local（含补全 + man）
```

带参数运行：`just run -- --help` 或 `just run-release -- --help`。

## 工具链

- Rust edition 2024，MSRV 1.91（`Cargo.toml` 的 `rust-version` 与 CI 的 msrv job 需保持一致）
- 无自定义 `rustfmt.toml` / `clippy.toml`，全部用工具默认
- 依赖保持最小：只加真正需要的 crate

## 代码结构

```
src/
├── main.rs    业务入口
└── cli.rs     clap 命令行定义（被 main.rs 和 build.rs 同时使用）
```

**`src/cli.rs` 是特殊文件**：`build.rs` 通过 `include!("src/cli.rs")` 复用它来生成 shell 补全和 man 手册页。因此该文件：

- 只能依赖 `clap` 和标准库
- 不能出现 `use crate::...`
- 改动后需要 `just build` 重新生成补全（build.rs 已配置 `rerun-if-changed`）

新增子命令时：在 `src/cli.rs` 里加 clap 定义 → `src/main.rs` 里分发 → `just build` 验证补全生成成功。

## 外围文件（已固定，不要改）

| 文件/目录 | 作用 |
|---|---|
| `build.rs` | 从 `src/cli.rs` 生成 shell 补全 + man（由 `SHELL_HELP_DIR` 控制） |
| `justfile` | 开发/安装/AUR 命令的唯一入口 |
| `.github/workflows/ci.yml` | 格式、clippy、测试、MSRV |
| `.github/workflows/build.yml` | 多平台交叉编译 + 打包（`SHELL_HELP_DIR` 环境变量已配好） |
| `.github/workflows/release.yml` | tag `v*` → GitHub Release → AUR 稳定版 |
| `.github/workflows/nightly.yml` | 每日 cron → 滚动 prerelease → AUR nightly |
| `aur/PKGBUILD`、`aur-nightly/PKGBUILD` | AUR 打包（`-bin` 包，下载 GitHub Release 资产） |
| `Cross.toml` | musl 交叉编译时透传 `SHELL_HELP_DIR` |
| `.editorconfig` | 缩进风格 |

需要改外围时（例如加平台、换 License、加 AUR 包名），**逐项说明理由后再改**，不要顺手重构。

## 发布流程

```bash
# 1. 改 Cargo.toml 的 version
# 2. 提交
git add -A && git commit -m "chore: release v0.1.0"
# 3. 打 tag 推上去，触发 release.yml
git tag v0.1.0 && git push origin v0.1.0
```

AUR 更新是自动的，前提是仓库 secret `AUR_SSH_PRIVATE_KEY` 已配置。

## 已知陷阱

- **不要手动设置 `SHELL_HELP_DIR` 去构建**：`just build` / `just install` 已经设置好；直接 `cargo build` 不会生成补全，但也无害。
- **AUR 禁止 force push**：justfile 用 clone → 复制 → push，不要改成 `git subtree push`。
- **nightly 版本号只在 `nightly.yml` 里算一次**，justfile 的 `aur-nightly-release` 用的是同一公式，改公式要两处同步。
- **打包资产名不含版本号**：PKGBUILD 下载时本地文件名带 `pkgver` 前缀，这是为了绕过 makepkg 缓存，别删。
- **`description` 不要包含双引号**：它会原样写进 `Cargo.toml`、PKGBUILD。

## 提交规范

`type: 中文描述`，type ∈ `feat` `fix` `refactor` `docs` `chore` `test` `perf`。

例：`feat: 支持从 stdin 读取`、`fix: 修复空文件处理`。
