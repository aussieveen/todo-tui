use super::todo::Todo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppFocus {
    Main,
    Popup,
    ErrorPopup,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupMode {
    Add,
    Edit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupField {
    Description,
    Priority,
    Context,
    Project,
    DueDate,
}

impl PopupField {
    pub fn next(self) -> Self {
        match self {
            Self::Description => Self::Priority,
            Self::Priority => Self::Context,
            Self::Context => Self::Project,
            Self::Project => Self::DueDate,
            Self::DueDate => Self::Description,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Description => Self::DueDate,
            Self::Priority => Self::Description,
            Self::Context => Self::Priority,
            Self::Project => Self::Context,
            Self::DueDate => Self::Project,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopupState {
    pub mode: PopupMode,
    pub field: PopupField,
    pub description: String,
    /// Single letter A–F, or empty for no priority.
    pub priority: String,
    /// Context without the @ prefix.
    pub context: String,
    /// Project without the + prefix.
    pub project: String,
    /// Due date as YYYY-MM-DD string, or empty.
    pub due_date: String,
    /// Index of the todo being edited (Edit mode only).
    pub edit_index: Option<usize>,
}

impl PopupState {
    pub fn for_add() -> Self {
        Self {
            mode: PopupMode::Add,
            field: PopupField::Description,
            description: String::new(),
            priority: String::new(),
            context: String::new(),
            project: String::new(),
            due_date: String::new(),
            edit_index: None,
        }
    }

    pub fn for_edit(index: usize, todo: &Todo) -> Self {
        Self {
            mode: PopupMode::Edit,
            field: PopupField::Description,
            description: plain_description(&todo.description),
            priority: todo.priority.map(|c| c.to_string()).unwrap_or_default(),
            context: todo.contexts.first().cloned().unwrap_or_default(),
            project: todo.projects.first().cloned().unwrap_or_default(),
            due_date: todo
                .due_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            edit_index: Some(index),
        }
    }

    /// Reference to the currently active field's string buffer.
    pub fn active_field_mut(&mut self) -> &mut String {
        match self.field {
            PopupField::Description => &mut self.description,
            PopupField::Priority => &mut self.priority,
            PopupField::Context => &mut self.context,
            PopupField::Project => &mut self.project,
            PopupField::DueDate => &mut self.due_date,
        }
    }

    /// Build the description line with @context, +project, and due: appended.
    pub fn full_description(&self) -> String {
        let mut desc = self.description.trim().to_string();
        if !self.context.trim().is_empty() {
            desc.push_str(&format!(" @{}", self.context.trim()));
        }
        if !self.project.trim().is_empty() {
            desc.push_str(&format!(" +{}", self.project.trim()));
        }
        let due = self.due_date.trim();
        if !due.is_empty() {
            desc.push_str(&format!(" due:{due}"));
        }
        desc
    }

    /// Priority A–E only; converts lowercase to uppercase.
    pub fn parsed_priority(&self) -> Option<char> {
        self.priority.trim().chars().next().and_then(|c| {
            let upper = c.to_ascii_uppercase();
            matches!(upper, 'A'..='E').then_some(upper)
        })
    }
}

/// Strip @context, +project, and due:... tokens from a description for editing.
fn plain_description(description: &str) -> String {
    description
        .split_whitespace()
        .filter(|w| !w.starts_with('@') && !w.starts_with('+') && !w.starts_with("due:"))
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug)]
pub struct AppState {
    pub todos: Vec<Todo>,
    /// Index into the flat visible list. `None` until the user first navigates.
    pub selected: Option<usize>,
    pub focus: AppFocus,
    pub show_completed: bool,
    pub popup: PopupState,
    pub error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            selected: None,
            focus: AppFocus::Main,
            show_completed: false,
            popup: PopupState::for_add(),
            error: None,
        }
    }

    /// Returns todos that should be visible, sorted by priority (A first, None last).
    /// Incomplete todos come first, then completed todos (if show_completed).
    pub fn visible_todos(&self) -> Vec<(usize, &Todo)> {
        let mut incomplete: Vec<(usize, &Todo)> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.done)
            .collect();

        incomplete.sort_by_key(|(_, t)| t.priority_order());

        if self.show_completed {
            let mut completed: Vec<(usize, &Todo)> = self
                .todos
                .iter()
                .enumerate()
                .filter(|(_, t)| t.done)
                .collect();
            completed.sort_by_key(|(_, t)| t.priority_order());
            incomplete.extend(completed);
        }

        incomplete
    }

    /// Total number of selectable items.
    pub fn selectable_count(&self) -> usize {
        self.visible_todos().len()
    }

    pub fn move_up(&mut self) {
        self.selected = match self.selected {
            Some(0) => None,
            Some(n) => Some(n - 1),
            None => None,
        };
    }

    pub fn move_down(&mut self) {
        let count = self.selectable_count();
        if count == 0 {
            return;
        }
        let max = count - 1;
        self.selected = Some(match self.selected {
            None => 0,
            Some(n) => (n + 1).min(max),
        });
    }

    /// Returns the original todos index for the currently selected item.
    pub fn selected_todo_index(&self) -> Option<usize> {
        self.selected
            .and_then(|pos| self.visible_todos().get(pos).map(|(i, _)| *i))
    }

    /// Clamp selection after a mutation that may have changed the visible count.
    pub fn clamp_selection(&mut self) {
        let count = self.selectable_count();
        self.selected = if count == 0 {
            None
        } else {
            self.selected.map(|n| n.min(count - 1))
        };
    }
}
