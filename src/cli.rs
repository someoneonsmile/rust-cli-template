// 命令行定义。
//
// 本文件同时被两处使用：
//   1. `src/main.rs` 里 `mod cli;`
//   2. `build.rs` 里 `include!("src/cli.rs");`（为了用 clap 生成补全和 man）
//
// 因此这里只能依赖 clap 与标准库，不能出现 `use crate::...`
// —— 否则 build.rs 编译会失败。

use clap::Parser;

/// {{ description }}
#[derive(Debug, Parser)]
#[command(name = "{{ project-name }}", version, about)]
pub struct Args {
    /// 输入文件路径（示例参数，按需替换）
    #[arg(value_name = "FILE")]
    pub file: Option<String>,

    /// 输出详细信息
    #[arg(short, long)]
    pub verbose: bool,
}
