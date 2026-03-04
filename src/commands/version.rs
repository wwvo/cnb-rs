//! version 子命令

/// 打印版本信息
pub fn run() {
    let version = env!("CARGO_PKG_VERSION");
    println!("cnb {version}");
}
