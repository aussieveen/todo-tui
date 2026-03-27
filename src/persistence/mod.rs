use std::{fs, path::PathBuf};

use chrono::Local;

use crate::{error::AppError, state::todo::Todo};

pub struct Persister {
    path: PathBuf,
    dir: PathBuf,
}

impl Persister {
    pub fn new() -> Result<Self, AppError> {
        let dir = dirs::home_dir()
            .ok_or_else(|| AppError::Load("cannot determine home directory".into()))?
            .join(".todo");

        fs::create_dir_all(&dir)
            .map_err(|e| AppError::Load(format!("cannot create ~/.todo: {e}")))?;

        Ok(Self {
            path: dir.join("todo.txt"),
            dir,
        })
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn load(&self) -> Result<Vec<Todo>, AppError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path).map_err(|e| AppError::Load(e.to_string()))?;

        Ok(Self::parse_content(&content))
    }

    /// Parse raw todo.txt content into todos (public so sync task can reuse it).
    pub fn parse_content(raw: &str) -> Vec<Todo> {
        raw.lines().filter_map(Todo::parse).collect()
    }

    pub fn save(&self, todos: &[Todo]) -> Result<(), AppError> {
        let content = todos
            .iter()
            .map(Todo::to_line)
            .collect::<Vec<_>>()
            .join("\n");

        let content = if content.is_empty() {
            content
        } else {
            format!("{content}\n")
        };

        fs::write(&self.path, content).map_err(|e| AppError::Save(e.to_string()))
    }

    /// Write raw text directly to todo.txt (used when pulling from Drive).
    pub fn write_raw(&self, content: &str) -> Result<(), AppError> {
        fs::write(&self.path, content).map_err(|e| AppError::Save(e.to_string()))
    }

    /// Serialise todos to their raw string form.
    pub fn serialise(todos: &[Todo]) -> String {
        let content = todos
            .iter()
            .map(Todo::to_line)
            .collect::<Vec<_>>()
            .join("\n");
        if content.is_empty() {
            content
        } else {
            format!("{content}\n")
        }
    }

    /// Save the last-synced version for conflict detection.
    pub fn save_base(&self, content: &str) -> Result<(), AppError> {
        fs::write(self.dir.join("last_sync.txt"), content)
            .map_err(|e| AppError::Save(e.to_string()))
    }

    /// Load the last-synced base version.
    #[allow(dead_code)]
    pub fn load_base(&self) -> Option<String> {
        fs::read_to_string(self.dir.join("last_sync.txt")).ok()
    }

    /// Write both sides of a conflict to ~/.todo/conflicts/ for audit.
    pub fn log_conflict(&self, local: &str, drive: &str) -> Result<(), AppError> {
        let conflicts_dir = self.dir.join("conflicts");
        fs::create_dir_all(&conflicts_dir)
            .map_err(|e| AppError::Save(format!("cannot create conflicts dir: {e}")))?;

        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let path = conflicts_dir.join(format!("{timestamp}.txt"));

        let content = format!(
            "=== LOCAL VERSION ===\n{local}\n\n=== DRIVE VERSION ===\n{drive}\n"
        );
        fs::write(path, content).map_err(|e| AppError::Save(e.to_string()))
    }
}
