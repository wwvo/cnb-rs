//! cnb repo settings branch-protection 子命令 — 分支保护规则管理

use anyhow::Result;
use clap::Parser;
use cnb_api::types::BranchProtectionRequest;
use cnb_core::context::AppContext;
use cnb_tui::confirm::confirm_action;
use cnb_tui::table::{Column, Table};
use cnb_tui::{info, success};

/// 分支保护规则管理
#[derive(Debug, Parser)]
pub struct BranchProtectionCommand {
    #[command(subcommand)]
    pub subcommand: BranchProtectionSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum BranchProtectionSubcommand {
    /// 列出分支保护规则
    List(BpListArgs),
    /// 查看规则详情
    Get(BpGetArgs),
    /// 创建分支保护规则
    Create(BpCreateArgs),
    /// 更新分支保护规则
    Update(BpUpdateArgs),
    /// 删除分支保护规则
    Delete(BpDeleteArgs),
}

impl BranchProtectionCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            BranchProtectionSubcommand::List(args) => run_list(ctx, args).await,
            BranchProtectionSubcommand::Get(args) => run_get(ctx, args).await,
            BranchProtectionSubcommand::Create(args) => run_create(ctx, args).await,
            BranchProtectionSubcommand::Update(args) => run_update(ctx, args).await,
            BranchProtectionSubcommand::Delete(args) => run_delete(ctx, args).await,
        }
    }
}

/// 列出分支保护规则
#[derive(Debug, Parser)]
pub struct BpListArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    pub repo: Option<String>,
}

/// 查看规则详情
#[derive(Debug, Parser)]
pub struct BpGetArgs {
    /// 规则 ID
    pub id: String,

    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,
}

/// 创建分支保护规则
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
pub struct BpCreateArgs {
    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 规则名称（支持通配符，如 main、release/*）
    #[arg(short, long)]
    pub rule: String,

    /// 允许所有人推送
    #[arg(long)]
    pub allow_pushes: bool,

    /// 仅允许管理员推送
    #[arg(long)]
    pub allow_master_pushes: bool,

    /// 允许所有人强制推送
    #[arg(long)]
    pub allow_force_pushes: bool,

    /// 仅允许管理员强制推送
    #[arg(long)]
    pub allow_master_force_pushes: bool,

    /// 必须通过 PR 推送代码
    #[arg(long)]
    pub require_pr: bool,

    /// 需要代码评审
    #[arg(long)]
    pub require_review: bool,

    /// 评审者数量 [1, 5]
    #[arg(long)]
    pub review_count: Option<u8>,

    /// 评审通过率 [1, 100]
    #[arg(long)]
    pub review_ratio: Option<u8>,

    /// 需管理员批准
    #[arg(long)]
    pub require_master_approve: bool,

    /// 需通过状态检查
    #[arg(long)]
    pub require_status_checks: bool,

    /// 仅允许线性提交
    #[arg(long)]
    pub require_linear_history: bool,
}

/// 更新分支保护规则
#[derive(Debug, Parser)]
pub struct BpUpdateArgs {
    /// 规则 ID
    pub id: String,

    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 规则名称
    #[arg(short, long)]
    pub rule: Option<String>,

    /// 允许所有人推送
    #[arg(long)]
    pub allow_pushes: Option<bool>,

    /// 仅允许管理员推送
    #[arg(long)]
    pub allow_master_pushes: Option<bool>,

    /// 允许所有人强制推送
    #[arg(long)]
    pub allow_force_pushes: Option<bool>,

    /// 仅允许管理员强制推送
    #[arg(long)]
    pub allow_master_force_pushes: Option<bool>,

    /// 必须通过 PR 推送代码
    #[arg(long)]
    pub require_pr: Option<bool>,

    /// 需要代码评审
    #[arg(long)]
    pub require_review: Option<bool>,

    /// 评审者数量 [1, 5]
    #[arg(long)]
    pub review_count: Option<u8>,

    /// 评审通过率 [1, 100]
    #[arg(long)]
    pub review_ratio: Option<u8>,

    /// 需管理员批准
    #[arg(long)]
    pub require_master_approve: Option<bool>,

    /// 需通过状态检查
    #[arg(long)]
    pub require_status_checks: Option<bool>,

    /// 仅允许线性提交
    #[arg(long)]
    pub require_linear_history: Option<bool>,
}

/// 删除分支保护规则
#[derive(Debug, Parser)]
pub struct BpDeleteArgs {
    /// 规则 ID
    pub id: String,

    /// 仓库路径（如 org/repo），不指定则使用当前仓库
    #[arg(long)]
    pub repo: Option<String>,

    /// 跳过确认提示
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub yes: bool,
}

fn bool_icon(v: bool) -> &'static str {
    if v { "✓" } else { "✗" }
}

async fn run_list(ctx: &AppContext, args: &BpListArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let rules = client.list_branch_protections(repo_path).await?;

    if rules.is_empty() {
        info!("没有分支保护规则");
        return Ok(());
    }

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&rules)?);
        return Ok(());
    }

    let mut table = Table::new(vec![
        Column::new("ID", 12),
        Column::new("规则", 15),
        Column::new("推送", 8),
        Column::new("强推", 8),
        Column::new("PR评审", 10),
        Column::new("状态检查", 8),
    ]);

    for rule in &rules {
        let push = if rule.allow_pushes {
            "所有人"
        } else if rule.allow_master_pushes {
            "管理员"
        } else {
            "✗"
        };

        let force = if rule.allow_force_pushes {
            "所有人"
        } else if rule.allow_master_force_pushes {
            "管理员"
        } else {
            "✗"
        };

        let review = if rule.required_pull_request_reviews {
            format!(
                "{} ({}人)",
                bool_icon(true),
                rule.required_approved_review_count
            )
        } else {
            bool_icon(false).to_string()
        };

        table.add_row(vec![
            rule.id.clone(),
            rule.rule.clone(),
            push.to_string(),
            force.to_string(),
            review,
            bool_icon(rule.required_status_checks).to_string(),
        ]);
    }

    table.print();

    Ok(())
}

async fn run_get(ctx: &AppContext, args: &BpGetArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let client = ctx.api_client()?;
    let bp = client.get_branch_protection(repo_path, &args.id).await?;

    if ctx.json() {
        println!("{}", serde_json::to_string_pretty(&bp)?);
        return Ok(());
    }

    println!("Rule:                    {}", bp.rule);
    println!("ID:                      {}", bp.id);
    println!();
    println!("推送权限:");
    println!("  允许所有人推送:         {}", bool_icon(bp.allow_pushes));
    println!(
        "  仅允许管理员推送:       {}",
        bool_icon(bp.allow_master_pushes)
    );
    println!();
    println!("强制推送:");
    println!(
        "  允许所有人强推:         {}",
        bool_icon(bp.allow_force_pushes)
    );
    println!(
        "  仅允许管理员强推:       {}",
        bool_icon(bp.allow_master_force_pushes)
    );
    println!();
    println!("分支管理:");
    println!(
        "  允许创建:               {}",
        if bp.allow_creation {
            "所有人"
        } else if bp.allow_master_creation {
            "仅管理员"
        } else {
            "✗"
        }
    );
    println!(
        "  允许删除:               {}",
        if bp.allow_deletions {
            "所有人"
        } else if bp.allow_master_deletions {
            "仅管理员"
        } else {
            "✗"
        }
    );
    println!();
    println!("合并控制:");
    println!(
        "  必须通过 PR:            {}",
        bool_icon(bp.required_must_push_via_pull_request)
    );
    println!(
        "  仅允许自动合并:         {}",
        bool_icon(bp.required_must_auto_merge)
    );
    println!(
        "  允许管理员手动合并:     {}",
        bool_icon(bp.allow_master_manual_merge)
    );
    println!();
    println!("代码评审:");
    println!(
        "  需要评审:               {}",
        bool_icon(bp.required_pull_request_reviews)
    );
    println!(
        "  评审者数量:             {}",
        bp.required_approved_review_count
    );
    println!(
        "  评审通过率:             {}%",
        bp.required_approved_review_ratio
    );
    println!(
        "  需管理员批准:           {}",
        bool_icon(bp.required_master_approve)
    );
    println!();
    println!("其他:");
    println!(
        "  仅允许线性提交:         {}",
        bool_icon(bp.required_linear_history)
    );
    println!(
        "  需通过状态检查:         {}",
        bool_icon(bp.required_status_checks)
    );

    Ok(())
}

async fn run_create(ctx: &AppContext, args: &BpCreateArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let req = BranchProtectionRequest {
        rule: Some(args.rule.clone()),
        allow_pushes: Some(args.allow_pushes),
        allow_master_pushes: Some(args.allow_master_pushes),
        allow_force_pushes: Some(args.allow_force_pushes),
        allow_master_force_pushes: Some(args.allow_master_force_pushes),
        required_must_push_via_pull_request: Some(args.require_pr),
        required_pull_request_reviews: Some(args.require_review),
        required_approved_review_count: args.review_count,
        required_approved_review_ratio: args.review_ratio,
        required_master_approve: Some(args.require_master_approve),
        required_status_checks: Some(args.require_status_checks),
        required_linear_history: Some(args.require_linear_history),
        ..Default::default()
    };

    let client = ctx.api_client()?;
    let bp = client.create_branch_protection(repo_path, &req).await?;

    success!("分支保护规则已创建 ({}, ID: {})", args.rule, bp.id);

    Ok(())
}

async fn run_update(ctx: &AppContext, args: &BpUpdateArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    let req = BranchProtectionRequest {
        rule: args.rule.clone(),
        allow_pushes: args.allow_pushes,
        allow_master_pushes: args.allow_master_pushes,
        allow_force_pushes: args.allow_force_pushes,
        allow_master_force_pushes: args.allow_master_force_pushes,
        required_must_push_via_pull_request: args.require_pr,
        required_pull_request_reviews: args.require_review,
        required_approved_review_count: args.review_count,
        required_approved_review_ratio: args.review_ratio,
        required_master_approve: args.require_master_approve,
        required_status_checks: args.require_status_checks,
        required_linear_history: args.require_linear_history,
        ..Default::default()
    };

    let client = ctx.api_client()?;
    client
        .update_branch_protection(repo_path, &args.id, &req)
        .await?;

    success!("分支保护规则已更新 ({})", args.id);

    Ok(())
}

async fn run_delete(ctx: &AppContext, args: &BpDeleteArgs) -> Result<()> {
    let repo_path = match &args.repo {
        Some(r) => r.as_str(),
        None => ctx.repo()?,
    };

    if !confirm_action(&format!("确认删除分支保护规则 {} ？", args.id), args.yes)? {
        info!("已取消");
        return Ok(());
    }

    let client = ctx.api_client()?;
    client.delete_branch_protection(repo_path, &args.id).await?;

    success!("分支保护规则已删除 ({})", args.id);

    Ok(())
}
