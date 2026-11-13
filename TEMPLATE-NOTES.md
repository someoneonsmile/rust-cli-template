# Rust CLI 模板 — 使用说明

一个可以「一行命令起手」的 Rust CLI 项目模板，外围基础设施全部预置好：

- clap derive 命令行 + `build.rs` 自动生成 bash/zsh/fish/elvish 补全与 man 页
- GitHub Actions：CI（fmt/clippy/test/MSRV）、6 平台交叉编译矩阵、tag 发布、每夜构建
- AUR：稳定版 + nightly 双包（`-bin`，自动更新）
- `justfile`：开发 / 安装 / 发布命令的唯一入口
- `AGENTS.md`：给 AI 编码代理的项目说明书，说明哪些是外围、不要动

## 生成新项目

```bash
cd ~/code
cargo generate rust-cli --name my-tool
```

（`rust-cli` 是 `~/.cargo/cargo-generate.toml` 里注册的 favorite；也可以用
`cargo generate --path ~/code/rust-cli-template --name my-tool`。）

生成时会依次询问：

| 变量 | 说明 |
|---|---|
| `description` | 一句话描述，会写进 `Cargo.toml`、`PKGBUILD`、clap about |
| `github_username` | GitHub 用户名；回车则用 `git config user.name` |
| `include_windows` | 是否发布 Windows 构建（影响编译矩阵） |
| `aur_package` | AUR 稳定包名；回车则用 `<项目名>-bin` |
| `aur_nightly_package` | AUR nightly 包名；回车则用 `<项目名>-nightly-bin` |

非交互/脚本化生成时用 `--define` 传全部变量：

```bash
cargo generate rust-cli --name my-tool \
  --define description="..." --define github_username="alice" \
  --define include_windows=true \
  --define aur_package="" --define aur_nightly_package=""
```

## 生成之后

1. 进目录 `just check`，确认能编译
2. 把 `AGENTS.md` 给 AI 看，然后只让它改 `src/`
3. 改 `Cargo.toml` 的 `version`，打 tag `v0.1.0` 推送，CI 自动发布

**首次发布前需要做的一件事**：配置 `AUR_SSH_PRIVATE_KEY` secret。可以让 AI 直接跑：

```bash
gh repo create <GitHub用户>/my-tool --private --source=. --push   # 仓库还没建时
just setup-secret <GitHub用户>/my-tool                             # 写入 AUR 私钥 secret
```

`just setup-secret` 内部是 `gh secret set AUR_SSH_PRIVATE_KEY -R <repo> < ~/.ssh/aur`，
私钥经 stdin 传入、不会被打印。前提是 `gh auth status` 已登录、AUR 私钥在 `~/.ssh/aur`。

**AUR 包不需要预先创建**：首次 push 时 AUR 会自动建包（clone 不存在的包名会得到空仓库），
所以 `-bin` 和 `-nightly-bin` 两个包都是第一次发布时自动出现的。

## 模板怎么维护

模板是从真实项目里抽出来的（clip / deref / ocat / subview / stow-cm / ringlog）。
当某个老项目的外围修了 bug、或你想统一行为时：

1. 在那个项目里验证改动（跑 `just ci`）
2. 把改动同步回本模板对应文件
3. 重新生成一个测试项目跑 `just ci` 验证

模板自身的文件改动要保证：生成结果里不再残留 `{{` / `{%`（GitHub Actions 的 `${{ }}` 除外）。

## 模板文件地图

| 路径 | 说明 |
|---|---|
| `cargo-generate.toml` | 模板变量定义 |
| `TEMPLATE-NOTES.md` | 本文件，不会复制到新项目 |
| `src/cli.rs` | clap 定义，**被 `build.rs` 和 `main.rs` 同时使用** |
| `build.rs` | 生成补全 + man，由 `SHELL_HELP_DIR` 控制 |
| `justfile` | 开发/安装/AUR 命令 |
| `.github/workflows/*` | CI / 编译矩阵 / 发布 / nightly |
| `aur/`、`aur-nightly/` | PKGBUILD + .SRCINFO |
