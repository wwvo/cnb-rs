//! version 子命令

use anyhow::Result;
use clap::Args;

/// 显示版本信息
#[derive(Debug, Args)]
pub struct VersionArgs;

impl VersionArgs {
    pub fn execute(&self) -> Result<()> {
        println!("{}", crate::build_info::COMMAND_LONG_VERSION);
        Ok(())
    }
}
