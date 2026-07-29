use clap::{Parser, Subcommand};

mod cli;
mod config;

use cli::{
    AnalyzeArgs, BuildArgs, PackageArgs, PublishArgs, ReleaseArgs, StoreArgs, UpgradeArgs,
    VersionCheckArgs, WorkflowArgs,
};
use fastforge_app_store_connect::cli::AppStoreConnectArgs;
use fastforge_google_play_console::cli::GooglePlayConsoleArgs;
use fastforge_skill_manager::cli::{SkillArgs, SkillCommand};

#[derive(Parser)]
#[command(name = "fastforge")]
#[command(about = "Package and publish your apps with ease.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Analyze your project")]
    Analyze(AnalyzeArgs),
    #[command(about = "Build your project")]
    Build(BuildArgs),
    #[command(about = "Package your project")]
    Package(PackageArgs),
    #[command(about = "Publish your project")]
    Publish(PublishArgs),
    #[command(about = "Release the current Flutter application")]
    Release(ReleaseArgs),
    #[command(about = "Manage distribution store configuration")]
    Store(StoreArgs),
    #[command(about = "Update Fastforge to the latest version")]
    Upgrade(UpgradeArgs),
    #[command(
        name = "version-check",
        about = "Check for a newer version of fastforge"
    )]
    VersionCheck(VersionCheckArgs),
    #[command(about = "Execute workflows locally")]
    Workflow(WorkflowArgs),
    #[command(name = "appstore", about = "Use App Store Connect")]
    AppStore(AppStoreConnectArgs),
    #[command(name = "googleplay", about = "Use Google Play Console")]
    GooglePlay(GooglePlayConsoleArgs),
    #[command(about = "Manage AI agent skills")]
    Skill(SkillArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze(args) => {
            cli::analyze::execute(args).await?;
        }
        Commands::Build(args) => {
            cli::build::execute(args).await?;
        }
        Commands::Package(args) => {
            cli::package::execute(args).await?;
        }
        Commands::Publish(args) => {
            cli::publish::execute(args).await?;
        }
        Commands::Release(args) => {
            cli::release::execute(args).await?;
        }
        Commands::Store(args) => {
            cli::store::execute(args).await?;
        }
        Commands::Upgrade(args) => {
            cli::upgrade::execute(args).await?;
        }
        Commands::VersionCheck(args) => {
            cli::version_check::execute(args).await?;
        }
        Commands::Workflow(args) => {
            cli::workflow::execute(args).await?;
        }
        Commands::AppStore(args) => {
            fastforge_app_store_connect::cli::execute(args).await?;
        }
        Commands::GooglePlay(args) => {
            fastforge_google_play_console::cli::execute(args).await?;
        }
        Commands::Skill(args) => {
            match &args.command {
                SkillCommand::Install(cmd_args) => fastforge_skill_manager::commands::install::execute(cmd_args).await?,
                SkillCommand::Update(cmd_args) => fastforge_skill_manager::commands::update::execute(cmd_args).await?,
                SkillCommand::Remove(cmd_args) => fastforge_skill_manager::commands::remove::execute(cmd_args).await?,
                SkillCommand::List(cmd_args) => fastforge_skill_manager::commands::list::execute(cmd_args).await?,
                SkillCommand::Inspect(cmd_args) => fastforge_skill_manager::commands::inspect::execute(cmd_args).await?,
            }
        }
    }

    Ok(())
}
