//! 成员管理子命令组

use anyhow::Result;
use clap::Parser;
use cnb_core::context::AppContext;

pub mod collaborator_list;
pub mod collaborator_remove;
pub mod collaborator_update;
pub mod group_access_level;
pub mod group_add;
pub mod group_inherited;
pub mod group_list;
pub mod group_remove;
pub mod group_update;
pub mod group_user_access;
pub mod repo_access_level;
pub mod repo_add;
pub mod repo_all;
pub mod repo_inherited;
pub mod repo_list;
pub mod repo_remove;
pub mod repo_update;
pub mod repo_user_access;

/// 成员管理
#[derive(Debug, Parser)]
pub struct MemberCommand {
    #[command(subcommand)]
    pub subcommand: MemberSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum MemberSubcommand {
    // === 仓库成员 ===
    /// 列出仓库直接成员
    #[command(name = "repo-list")]
    RepoList(repo_list::RepoListArgs),
    /// 添加仓库成员
    #[command(name = "repo-add")]
    RepoAdd(repo_add::RepoAddArgs),
    /// 更新仓库成员权限
    #[command(name = "repo-update")]
    RepoUpdate(repo_update::RepoUpdateArgs),
    /// 移除仓库成员
    #[command(name = "repo-remove")]
    RepoRemove(repo_remove::RepoRemoveArgs),
    /// 查看自己在仓库的权限
    #[command(name = "repo-access-level")]
    RepoAccessLevel(repo_access_level::RepoAccessLevelArgs),
    /// 查看指定成员在仓库的权限层级
    #[command(name = "repo-user-access")]
    RepoUserAccess(repo_user_access::RepoUserAccessArgs),
    /// 列出仓库继承成员
    #[command(name = "repo-inherited")]
    RepoInherited(repo_inherited::RepoInheritedArgs),
    /// 列出仓库所有有效成员
    #[command(name = "repo-all")]
    RepoAll(repo_all::RepoAllArgs),

    // === 组织成员 ===
    /// 列出组织直接成员
    #[command(name = "group-list")]
    GroupList(group_list::GroupListArgs),
    /// 添加组织成员
    #[command(name = "group-add")]
    GroupAdd(group_add::GroupAddArgs),
    /// 更新组织成员权限
    #[command(name = "group-update")]
    GroupUpdate(group_update::GroupUpdateArgs),
    /// 移除组织成员
    #[command(name = "group-remove")]
    GroupRemove(group_remove::GroupRemoveArgs),
    /// 查看自己在组织的权限
    #[command(name = "group-access-level")]
    GroupAccessLevel(group_access_level::GroupAccessLevelArgs),
    /// 查看指定成员在组织的权限层级
    #[command(name = "group-user-access")]
    GroupUserAccess(group_user_access::GroupUserAccessArgs),
    /// 列出组织继承成员
    #[command(name = "group-inherited")]
    GroupInherited(group_inherited::GroupInheritedArgs),

    // === 外部贡献者 ===
    /// 列出外部贡献者
    #[command(name = "collaborator-list")]
    CollaboratorList(collaborator_list::CollaboratorListArgs),
    /// 更新外部贡献者权限
    #[command(name = "collaborator-update")]
    CollaboratorUpdate(collaborator_update::CollaboratorUpdateArgs),
    /// 移除外部贡献者
    #[command(name = "collaborator-remove")]
    CollaboratorRemove(collaborator_remove::CollaboratorRemoveArgs),
}

impl MemberCommand {
    pub async fn execute(&self, ctx: &AppContext) -> Result<()> {
        match &self.subcommand {
            MemberSubcommand::RepoList(args) => repo_list::run(ctx, args).await,
            MemberSubcommand::RepoAdd(args) => repo_add::run(ctx, args).await,
            MemberSubcommand::RepoUpdate(args) => repo_update::run(ctx, args).await,
            MemberSubcommand::RepoRemove(args) => repo_remove::run(ctx, args).await,
            MemberSubcommand::RepoAccessLevel(args) => repo_access_level::run(ctx, args).await,
            MemberSubcommand::RepoUserAccess(args) => repo_user_access::run(ctx, args).await,
            MemberSubcommand::RepoInherited(args) => repo_inherited::run(ctx, args).await,
            MemberSubcommand::RepoAll(args) => repo_all::run(ctx, args).await,
            MemberSubcommand::GroupList(args) => group_list::run(ctx, args).await,
            MemberSubcommand::GroupAdd(args) => group_add::run(ctx, args).await,
            MemberSubcommand::GroupUpdate(args) => group_update::run(ctx, args).await,
            MemberSubcommand::GroupRemove(args) => group_remove::run(ctx, args).await,
            MemberSubcommand::GroupAccessLevel(args) => group_access_level::run(ctx, args).await,
            MemberSubcommand::GroupUserAccess(args) => group_user_access::run(ctx, args).await,
            MemberSubcommand::GroupInherited(args) => group_inherited::run(ctx, args).await,
            MemberSubcommand::CollaboratorList(args) => collaborator_list::run(ctx, args).await,
            MemberSubcommand::CollaboratorUpdate(args) => collaborator_update::run(ctx, args).await,
            MemberSubcommand::CollaboratorRemove(args) => collaborator_remove::run(ctx, args).await,
        }
    }
}
