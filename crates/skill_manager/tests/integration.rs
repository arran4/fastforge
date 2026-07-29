use std::fs;
use std::path::PathBuf;
use fastforge_skill_manager::manager::SkillManager;
use fastforge_skill_manager::target::{AgentTarget, SkillScope};

#[tokio::test]
async fn test_skill_lifecycle() -> anyhow::Result<()> {
    // We will test by pretending to install from a local path
    let test_workspace = tempfile::tempdir()?;
    let skill_src = test_workspace.path().join("my-skill");
    fs::create_dir_all(&skill_src)?;
    fs::write(skill_src.join("SKILL.md"), "# Test Skill\n")?;

    let manager = SkillManager::new(AgentTarget::Common, SkillScope::Project);

    // Override the target path for tests
    std::env::set_current_dir(test_workspace.path())?;

    // Install
    manager.install(&skill_src.to_string_lossy(), None).await?;

    // Check it's installed
    let skills = manager.get_skills_dir()?;
    assert!(skills.join("my-skill").exists());
    assert!(skills.join("my-skill").join("SKILL.md").exists());
    assert!(skills.join("my-skill").join(".skill-metadata.json").exists());

    // Update (no changes, should just print up to date)
    manager.update(Some("my-skill"), false, false).await?;

    // Remove
    manager.remove("my-skill")?;
    assert!(!skills.join("my-skill").exists());

    Ok(())
}
