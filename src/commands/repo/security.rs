//! cnb repo security 子命令 — 查看仓库安全概览

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;
use cnb_tui::info;
use cnb_tui::table::{Column, Table};

/// 查看仓库安全模块概览
#[derive(Debug, Parser)]
pub struct SecurityArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,

    /// 扫描类型过滤（逗号分隔：`code_sensitive`,`code_vulnerability`,`code_issue`）
    #[arg(short, long)]
    pub types: Option<String>,

    /// 查询类型：open/ignore/all（默认 all）
    #[arg(long)]
    pub tab: Option<String>,
}

pub async fn run(ctx: &AppContext, args: &SecurityArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let overview = client
        .get_security_overview(repo_path, args.types.as_deref(), args.tab.as_deref())
        .await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&overview)?);
        return Ok(());
    }

    // 风险总览
    if let Some(risk) = &overview.risk_cnt {
        if risk.enable {
            info!("风险总数：{}", risk.total);
        } else {
            info!("安全扫描未启用");
            return Ok(());
        }
    }

    let mut table = Table::new(vec![
        Column::new("扫描类型", 18),
        Column::new("状态", 6),
        Column::new("开启", 6),
        Column::new("忽略", 6),
        Column::new("严重", 6),
        Column::new("高", 6),
        Column::new("中", 6),
        Column::new("低", 6),
    ]);

    // 代码敏感信息
    if let Some(cs) = &overview.code_sensitive {
        let status = if cs.enable { "已启用" } else { "未启用" };
        table.add_row(vec![
            "敏感信息".to_string(),
            status.to_string(),
            cs.open.to_string(),
            cs.ignored.to_string(),
            cs.critical_count.to_string(),
            cs.high_count.to_string(),
            cs.medium_count.to_string(),
            cs.low_count.to_string(),
        ]);
    }

    // 代码漏洞
    if let Some(cv) = &overview.code_vulnerability {
        let status = if cv.enable { "已启用" } else { "未启用" };
        table.add_row(vec![
            "代码漏洞".to_string(),
            status.to_string(),
            cv.open.to_string(),
            cv.ignored.to_string(),
            cv.critical_vul_open_cnt.to_string(),
            cv.high_vul_open_cnt.to_string(),
            cv.medium_vul_open_cnt.to_string(),
            cv.low_vul_open_cnt.to_string(),
        ]);
    }

    // 代码问题
    if let Some(ci) = &overview.code_issue {
        let status = if ci.enable { "已启用" } else { "未启用" };
        table.add_row(vec![
            "代码问题".to_string(),
            status.to_string(),
            ci.open.to_string(),
            ci.ignored.to_string(),
            ci.critical_count.to_string(),
            ci.high_count.to_string(),
            ci.medium_count.to_string(),
            ci.low_count.to_string(),
        ]);
    }

    table.print();

    Ok(())
}
