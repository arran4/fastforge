use crate::models::{InstalledSkill, SkillMetadata};
use crate::source::{compute_digest, SkillSource};
use crate::target::{AgentTarget, SkillScope};
use anyhow::{anyhow, bail, Context, Result};
use chrono::Utc;
use reqwest;
use serde_json;
use std::fs;
use std::path::Component;
use std::path::{Path, PathBuf};

pub struct SkillManager {
    target: AgentTarget,
    scope: SkillScope,
}

impl SkillManager {
    pub fn new(target: AgentTarget, scope: SkillScope) -> Self {
        Self { target, scope }
    }

    pub fn get_skills_dir(&self) -> Result<PathBuf> {
        self.target.get_path(&self.scope)
    }

    pub async fn install(&self, source_str: &str, name_or_path: Option<&str>) -> Result<()> {
        let source = SkillSource::parse(source_str, name_or_path);
        let skills_dir = self.get_skills_dir()?;

        let final_name = if let Some(n) = name_or_path {
            n.split('/').last().unwrap_or(n) // If a path was provided, use the last component as the default name. If it's just a name, it remains the name.
        } else {
            // If no name provided, extract it from the source (e.g. owner/repo -> repo, ./my-skill -> my-skill)
            source_str.split('/').last().unwrap_or(source_str)
        };

        if is_unsafe_path(&final_name) {
            bail!("Unsafe skill name: '{}'", final_name);
        }
        fs::create_dir_all(&skills_dir).context("Failed to create skills directory")?;

        let (temp_dir, original_source, source_type, remote_id, source_path, name, revision) =
            self.fetch_source(&source).await?;

        let skill_md_path = self.find_skill_md(&temp_dir)?;
        let skill_root = skill_md_path.parent().unwrap();

        let dest_dir = skills_dir.join(final_name);

        if dest_dir.exists() {
            bail!("Skill '{}' is already installed. Use update or remove first.", final_name);
        }

        self.copy_dir_safe(skill_root, &dest_dir)?;

        let metadata = SkillMetadata {
            original_source,
            source_type,
            remote_identifier: remote_id,
            source_path,
            skill_name: final_name.to_string(),
            revision,
            installed_at: Utc::now().to_rfc3339(),
            installer_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let metadata_path = dest_dir.join(".skill-metadata.json");
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_path, metadata_json).context("Failed to write metadata")?;

        println!("Successfully installed skill '{}'", final_name);

        Ok(())
    }

    pub async fn update(&self, name: Option<&str>, all: bool, force: bool) -> Result<()> {
        let skills_dir = self.get_skills_dir()?;
        if !skills_dir.exists() {
            bail!("No skills directory found.");
        }

        let skills_to_update = if all {
            self.list_installed()?
        } else if let Some(n) = name {
            let s = self.get_installed(n)?;
            vec![s]
        } else {
            bail!("Must specify a skill name or --all");
        };

        for skill in skills_to_update {
            println!("Checking for updates for '{}'...", skill.name);
            let metadata = &skill.metadata;
            let source_str = metadata.original_source.clone();
            let source = SkillSource::parse(&source_str, metadata.source_path.as_deref());

            let (temp_dir, _, _, _, _, _, new_revision) = self.fetch_source(&source).await?;

            if metadata.revision == new_revision {
                println!("Skill '{}' is already up to date.", skill.name);
                continue;
            }

            let current_digest = compute_digest(Path::new(&skill.installation_path))?;
            if !force && current_digest != metadata.revision {
                 println!("skill has local modifications; use --force to replace");
                 continue;
            }

            let skill_md_path = self.find_skill_md(&temp_dir)?;
            let skill_root = skill_md_path.parent().unwrap();

            let dest_dir = PathBuf::from(&skill.installation_path);
            if dest_dir.exists() {
                fs::remove_dir_all(&dest_dir)?;
            }
            self.copy_dir_safe(skill_root, &dest_dir)?;

            let mut new_metadata = metadata.clone();
            new_metadata.revision = new_revision;
            new_metadata.installed_at = Utc::now().to_rfc3339();

            let metadata_path = dest_dir.join(".skill-metadata.json");
            let metadata_json = serde_json::to_string_pretty(&new_metadata)?;
            fs::write(&metadata_path, metadata_json).context("Failed to write metadata")?;

            println!("Successfully updated skill '{}'", skill.name);
        }

        Ok(())
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        let skills_dir = self.get_skills_dir()?;

        if is_unsafe_path(name) {
            bail!("Unsafe skill name: '{}'", name);
        }
        let dest_dir = skills_dir.join(name);

        if !dest_dir.exists() {
            bail!("Skill '{}' not found", name);
        }

        fs::remove_dir_all(&dest_dir)?;
        println!("Removed skill '{}'", name);
        Ok(())
    }

    pub fn list(&self, machine_readable: bool) -> Result<()> {
        let skills = self.list_installed()?;
        if machine_readable {
            println!("{}", serde_json::to_string_pretty(&skills)?);
        } else {
            for skill in skills {
                println!("- {} (Source: {})", skill.name, skill.metadata.original_source);
            }
        }
        Ok(())
    }

    pub fn inspect(&self, name: &str) -> Result<()> {
        let skill = self.get_installed(name)?;
        println!("Name: {}", skill.name);
        println!("Path: {}", skill.installation_path);
        println!("Source: {}", skill.metadata.original_source);
        println!("Revision: {}", skill.metadata.revision);
        println!("Installed At: {}", skill.metadata.installed_at);
        Ok(())
    }

    fn list_installed(&self) -> Result<Vec<InstalledSkill>> {
        let skills_dir = self.get_skills_dir()?;
        if !skills_dir.exists() {
            return Ok(vec![]);
        }

        let mut skills = Vec::new();
        for entry in fs::read_dir(skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Ok(skill) = self.read_installed_skill(&path) {
                    skills.push(skill);
                }
            }
        }
        Ok(skills)
    }

    fn get_installed(&self, name: &str) -> Result<InstalledSkill> {
        let skills_dir = self.get_skills_dir()?;
        let dest_dir = skills_dir.join(name);
        self.read_installed_skill(&dest_dir).map_err(|_| anyhow!("Skill '{}' not found", name))
    }

    fn read_installed_skill(&self, path: &Path) -> Result<InstalledSkill> {
        let metadata_path = path.join(".skill-metadata.json");
        let metadata_content = fs::read_to_string(metadata_path)?;
        let metadata: SkillMetadata = serde_json::from_str(&metadata_content)?;
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        Ok(InstalledSkill {
            name,
            metadata,
            installation_path: path.to_string_lossy().to_string(),
        })
    }

    async fn fetch_source(&self, source: &SkillSource) -> Result<(PathBuf, String, String, Option<String>, Option<String>, String, String)> {
        let temp_dir = std::env::temp_dir().join(format!("fastforge_skill_{}", uuid::Uuid::new_v4()));

        match source {
            SkillSource::Local { path } => {
                if !path.exists() {
                    bail!("Local path does not exist");
                }
                let digest = compute_digest(path)?;
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                self.copy_dir_safe(path, &temp_dir)?;

                Ok((
                    temp_dir,
                    path.to_string_lossy().to_string(),
                    "local".to_string(),
                    None,
                    None,
                    name,
                    digest
                ))
            },
            SkillSource::GitHub { owner, repo, path: opt_path } => {
                let url = format!("https://api.github.com/repos/{}/{}/tarball", owner, repo);

                let client = reqwest::Client::builder()
                    .user_agent("fastforge-cli")
                    .build()?;
                let resp = client.get(&url).send().await?.error_for_status()?;
                let bytes = resp.bytes().await?;

                let tar_gz = temp_dir.with_extension("tar.gz");
                fs::write(&tar_gz, &bytes)?;

                fs::create_dir_all(&temp_dir)?;

                let tar_gz_file = fs::File::open(&tar_gz)?;
                let decompressed = flate2::read::GzDecoder::new(tar_gz_file);
                let mut archive = tar::Archive::new(decompressed);
                archive.unpack(&temp_dir)?;

                let mut target_dir;
                let mut root_dir = None;
                for entry in fs::read_dir(&temp_dir)? {
                    let entry = entry?;
                    if entry.path().is_dir() && entry.file_name() != ".git" {
                        root_dir = Some(entry.path());
                        break;
                    }
                }

                let base_dir = root_dir.unwrap_or(temp_dir);
                if let Some(p) = opt_path {
                    target_dir = base_dir.join(p);
                } else {
                    target_dir = base_dir;
                }

                let digest = compute_digest(&target_dir)?;
                let name = target_dir.file_name().unwrap().to_string_lossy().to_string();

                Ok((
                    target_dir,
                    format!("{}/{}", owner, repo),
                    "github".to_string(),
                    Some(format!("{}/{}", owner, repo)),
                    opt_path.clone(),
                    name,
                    digest
                ))
            }
        }
    }

    fn find_skill_md(&self, path: &Path) -> Result<PathBuf> {
        if path.is_file() {
            if path.file_name().unwrap() == "SKILL.md" {
                return Ok(path.to_path_buf());
            }
        } else {
            let md_path = path.join("SKILL.md");
            if md_path.exists() {
                return Ok(md_path);
            }
            for entry in walkdir::WalkDir::new(path) {
                let entry = entry?;
                if entry.file_name() == "SKILL.md" {
                    return Ok(entry.into_path());
                }
            }
        }
        bail!("SKILL.md not found in source");
    }

    fn copy_dir_safe(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        for entry in walkdir::WalkDir::new(src) {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let rel = path.strip_prefix(src)?;
                let target = dst.join(rel);

                if let Some(p) = target.parent() {
                    fs::create_dir_all(p)?;
                }
                fs::copy(path, target)?;
            }
        }
        Ok(())
    }
}

fn is_unsafe_path(path: &str) -> bool {
    let p = Path::new(path);
    p.components().any(|c| match c {
        Component::ParentDir | Component::RootDir | Component::Prefix(_) => true,
        _ => false,
    }) || path.contains("..") || path.contains("/") || path.contains("\\")
}
