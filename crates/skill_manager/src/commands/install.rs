use crate::manager::SkillManager;
use crate::target::{AgentTarget, SkillScope};
use clap::Args;

#[derive(Args)]
pub struct InstallArgs {
    #[arg(help = "The source of the skill (e.g., owner/repo, owner/repo path/to/skill, ./local)")]
    pub source: String,

    #[arg(help = "Optional specific path within a repo, or the final name for the skill")]
    pub name_or_path: Option<String>,

    #[arg(long, default_value = "user", help = "Scope for installation (user or project)")]
    pub scope: String,

    #[arg(long, default_value = "common", help = "Target agent (codex, claude, copilot, cursor, common)")]
    pub agent: String,
}

pub async fn execute(args: &InstallArgs) -> anyhow::Result<()> {
    let scope = match args.scope.to_lowercase().as_str() {
        "project" => SkillScope::Project,
        _ => SkillScope::User,
    };

    let target = AgentTarget::parse(&args.agent).unwrap_or(AgentTarget::Common);

    let manager = SkillManager::new(target, scope);
    manager.install(&args.source, args.name_or_path.as_deref()).await
}
