# {{ project-name }}

{{ description }}

## 安装

从源码：

```bash
just install          # 构建 release 并安装到 /usr/local/bin（含 bash/zsh/fish 补全 + man）
just uninstall        # 卸载
```

Arch Linux（AUR，发布第一个版本后可用）：

```bash
# yay
yay -S {% if aur_package != "" %}{{ aur_package }}{% else %}{{ project-name }}-bin{% endif %}          # 稳定版
yay -S {% if aur_nightly_package != "" %}{{ aur_nightly_package }}{% else %}{{ project-name }}-nightly-bin{% endif %}   # 每夜构建

# paru
paru -S {% if aur_package != "" %}{{ aur_package }}{% else %}{{ project-name }}-bin{% endif %}
paru -S {% if aur_nightly_package != "" %}{{ aur_nightly_package }}{% else %}{{ project-name }}-nightly-bin{% endif %}
```

## 使用

```bash
{{ project-name }} --help
{{ project-name }} --verbose FILE
```

## 开发

所有命令统一走 `just`（`just --list` 查看全部）：

```bash
just check        # cargo check
just build        # debug 构建（同时生成 shell 补全到 shell_help/）
just release      # release 构建
just test         # 测试
just fmt          # 格式化
just lint-strict  # clippy -D warnings
just ci           # 全量检查：fmt-check → lint-strict → test → release
just run -- --help
```

## 发布

```bash
# 1. 改 Cargo.toml 里的 version
# 2. 提交后打 tag 并推送
git tag v0.1.0
git push origin v0.1.0
```

推送 tag 后 GitHub Actions 会自动：

1. 交叉编译 6 个平台（Linux gnu/musl、Linux arm64、macOS x86_64/arm64、Windows）
2. 创建 GitHub Release（带分类 changelog）
3. 更新 AUR 稳定版包

`nightly` 分支的每日构建由 `nightly.yml` 负责，产物是滚动 prerelease + AUR nightly 包。

**前置条件**：仓库需要 `AUR_SSH_PRIVATE_KEY` secret。用 `just setup-secret <owner>/<repo>` 一键写入
（内部是 `gh secret set ... < ~/.ssh/aur`，私钥不会打印）。AUR 包无需预先创建，首次推送自动建包。

## License

MIT
