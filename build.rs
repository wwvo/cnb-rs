fn command_output(program: &str, args: &[&str]) -> String {
    std::process::Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map_or_else(
            || "unknown".to_string(),
            |output| String::from_utf8_lossy(&output.stdout).trim().to_string(),
        )
}

fn main() {
    // 目标平台
    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=TARGET={target}");

    // 构建 profile
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={profile}");

    // Git commit hash（短）
    let git_hash = command_output("git", &["rev-parse", "--short=7", "HEAD"]);
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    // Git commit 日期
    let git_date = command_output("git", &["log", "-1", "--format=%cs"]);
    println!("cargo:rustc-env=GIT_DATE={git_date}");

    // rustc 版本
    let rustc_version = command_output("rustc", &["--version"]);
    println!("cargo:rustc-env=RUSTC_VERSION_INFO={rustc_version}");
}
