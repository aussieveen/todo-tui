use std::{fs, path::PathBuf};

use crate::{error::AppError, state::todo::Todo};

pub struct Persister {
    path: PathBuf,
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
        })
    }

    pub fn load(&self) -> Result<Vec<Todo>, AppError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path).map_err(|e| AppError::Load(e.to_string()))?;

        Ok(content.lines().filter_map(Todo::parse).collect())
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
}
