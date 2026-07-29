use crate::manager::SkillManager;
use crate::target::{AgentTarget, SkillScope};
use clap::Args;

#[derive(Args)]
pub struct InspectArgs {
    #[arg(help = "Name of the skill to inspect")]
    pub name: String,

    #[arg(long, default_value = "user", help = "Scope (user or project)")]
    pub scope: String,

    #[arg(long, default_value = "common", help = "Target agent (codex, claude, copilot, cursor, common)")]
    pub agent: String,
}

pub async fn execute(args: &InspectArgs) -> anyhow::Result<()> {
    let scope = match args.scope.to_lowercase().as_str() {
        "project" => SkillScope::Project,
        _ => SkillScope::User,
    };

    let target = AgentTarget::parse(&args.agent).unwrap_or(AgentTarget::Common);

    let manager = SkillManager::new(target, scope);
    manager.inspect(&args.name)
}
