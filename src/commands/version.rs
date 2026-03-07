//! version 子命令

/// 打印详细版本信息
pub fn run() {
    let version = env!("CARGO_PKG_VERSION");
    let git_hash = env!("GIT_HASH");
    let git_date = env!("GIT_DATE");
    let rustc_version = env!("RUSTC_VERSION_INFO");
    let target = env!("TARGET");
    let profile = env!("BUILD_PROFILE");

    println!("cnb {version} ({git_hash} {git_date})");
    println!("{rustc_version}");
    println!("target: {target}");
    println!("profile: {profile}");
}
