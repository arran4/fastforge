use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub original_source: String,
    pub source_type: String,
    pub remote_identifier: Option<String>,
    pub source_path: Option<String>,
    pub skill_name: String,
    pub revision: String,
    pub installed_at: String,
    pub installer_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub name: String,
    pub metadata: SkillMetadata,
    pub installation_path: String,
}
