//! browse 子命令 - 在浏览器中打开仓库页面

use anyhow::Result;
use clap::Args;
use cnb_core::context::AppContext;

/// 在浏览器中打开仓库页面
#[derive(Debug, Args)]
pub struct BrowseArgs {
    /// 子路径（如 -/issues、-/pulls、-/settings）
    #[arg(default_value = "")]
    pub path: String,
}

impl BrowseArgs {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let url = ctx.web_url(&self.path)?;
        eprintln!("正在打开 {url}");
        AppContext::open_in_browser(&url)
    }
}
