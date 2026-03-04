//! 终端命令行补全脚本生成

use clap::CommandFactory;
use clap_complete::{Shell, generate};

/// 生成指定 shell 的命令行补全脚本并输出到 stdout
pub fn run(shell: Shell) {
    let mut cmd = crate::Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, &bin_name, &mut std::io::stdout());
}
