use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillScope {
    User,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentTarget {
    Common,
    Codex,
    Claude,
    Copilot,
    Cursor,
}

impl AgentTarget {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "codex" => Some(AgentTarget::Codex),
            "claude" => Some(AgentTarget::Claude),
            "copilot" => Some(AgentTarget::Copilot),
            "cursor" => Some(AgentTarget::Cursor),
            "common" => Some(AgentTarget::Common),
            _ => None,
        }
    }

    pub fn get_path(&self, scope: &SkillScope) -> anyhow::Result<PathBuf> {
        let base_dir = match scope {
            SkillScope::Project => PathBuf::from("."),
            SkillScope::User => {
                dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            }
        };

        // Share the common implementation for `.agents/skills/` for most
        let relative_path = match self {
            AgentTarget::Common | AgentTarget::Codex | AgentTarget::Claude => ".agents/skills",
            AgentTarget::Copilot => ".github/copilot/skills",
            AgentTarget::Cursor => ".cursor/skills",
        };

        Ok(base_dir.join(relative_path))
    }
}
