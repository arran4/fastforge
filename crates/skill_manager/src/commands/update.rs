use crate::manager::SkillManager;
use crate::target::{AgentTarget, SkillScope};
use clap::Args;

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(help = "Name of the skill to update")]
    pub name: Option<String>,

    #[arg(long, help = "Update all installed skills")]
    pub all: bool,

    #[arg(long, help = "Force update even if locally modified")]
    pub force: bool,

    #[arg(long, default_value = "user", help = "Scope (user or project)")]
    pub scope: String,

    #[arg(long, default_value = "common", help = "Target agent (codex, claude, copilot, cursor, common)")]
    pub agent: String,
}

pub async fn execute(args: &UpdateArgs) -> anyhow::Result<()> {
    let scope = match args.scope.to_lowercase().as_str() {
        "project" => SkillScope::Project,
        _ => SkillScope::User,
    };

    let target = AgentTarget::parse(&args.agent).unwrap_or(AgentTarget::Common);

    let manager = SkillManager::new(target, scope);
    manager.update(args.name.as_deref(), args.all, args.force).await
}
