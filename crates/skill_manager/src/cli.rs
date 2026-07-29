use clap::{Args, Subcommand};

#[derive(Args)]
pub struct SkillArgs {
    #[command(subcommand)]
    pub command: SkillCommand,
}

#[derive(Subcommand)]
pub enum SkillCommand {
    #[command(about = "Install a skill")]
    Install(crate::commands::install::InstallArgs),
    #[command(about = "Update installed skills")]
    Update(crate::commands::update::UpdateArgs),
    #[command(about = "Remove a skill")]
    Remove(crate::commands::remove::RemoveArgs),
    #[command(about = "List installed skills")]
    List(crate::commands::list::ListArgs),
    #[command(about = "Inspect an installed skill")]
    Inspect(crate::commands::inspect::InspectArgs),
}
