use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone)]
pub enum SkillSource {
    Local { path: PathBuf },
    GitHub { owner: String, repo: String, path: Option<String> },
}

impl SkillSource {
    pub fn parse(source: &str, path_arg: Option<&str>) -> Self {
        if source.starts_with(".") || source.starts_with("/") || source.starts_with("~") || source.contains("\\") || Path::new(source).exists() {
            SkillSource::Local {
                path: PathBuf::from(source),
            }
        } else if let Some((owner, repo)) = source.split_once('/') {
            SkillSource::GitHub {
                owner: owner.to_string(),
                repo: repo.to_string(),
                path: path_arg.map(|s| s.to_string()),
            }
        } else {
            // Default fallback if ambiguous, treat as local path or error
            SkillSource::Local {
                path: PathBuf::from(source),
            }
        }
    }
}

pub fn compute_digest(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    if path.is_file() {
        let content = std::fs::read(path)?;
        hasher.update(&content);
    } else {
        // Simple directory digest: combine digests of files
        let mut entries: Vec<_> = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file() && e.file_name() != ".skill-metadata.json") // exclude sidecar
            .collect();
        entries.sort_by(|a, b| a.path().cmp(b.path()));

        for entry in entries {
            let content = std::fs::read(entry.path())?;
            hasher.update(entry.path().strip_prefix(path).unwrap_or(entry.path()).to_string_lossy().as_bytes());
            hasher.update(&content);
        }
    }
    Ok(hex::encode(hasher.finalize()))
}
