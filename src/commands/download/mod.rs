//! Download 子命令 - 从仓库下载文件

use clap::Parser;

pub mod run;

/// 从仓库下载原始文件
#[derive(Debug, Parser)]
#[command(name = "download")]
pub struct DownloadArgs {
    /// 要下载的文件路径（逗号分隔或多次指定）
    #[arg(long = "files", value_delimiter = ',')]
    pub files: Vec<String>,

    /// Git 引用（分支/tag/commit），默认使用默认分支
    #[arg(long = "ref", default_value = "")]
    pub git_ref: String,

    /// 本地下载目录
    #[arg(long = "local-dir", default_value = ".")]
    pub local_dir: String,

    /// 包含的文件 glob 模式（可多次指定）
    #[arg(long = "include")]
    pub include: Vec<String>,

    /// 排除的文件 glob 模式（可多次指定）
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,

    /// 最大并发下载数
    #[arg(short = 'c', long = "concurrency", default_value_t = 4)]
    pub concurrency: usize,
}
