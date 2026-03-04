//! cnb config list 子命令

use cnb_core::config::{Config, DEFAULT_DOMAIN, DEFAULT_SCHEME};
use cnb_core::context::AppContext;

/// 配置项默认值
fn default_value(key: &str) -> &str {
    match key {
        "domain" => DEFAULT_DOMAIN,
        "git_protocol" => DEFAULT_SCHEME,
        _ => "",
    }
}

/// 列出所有配置项
pub fn run(ctx: &AppContext) {
    let config = ctx.config();

    for key in Config::VALID_KEYS {
        match config.get_value(key) {
            Some(val) => println!("{key} = {val}"),
            None => println!("{key} = {} (default)", default_value(key)),
        }
    }
}
