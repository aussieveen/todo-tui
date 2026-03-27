use crossterm::event::Event as CrosstermEvent;

use crate::sync::SyncStatus;

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
    App(AppEvent),
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    // Navigation
    MoveUp,
    MoveDown,
    // Popup triggers
    OpenAddPopup,
    OpenEditPopup,
    // Popup field input
    NextField,
    PrevField,
    AddChar(char),
    RemoveChar,
    SubmitPopup,
    CancelPopup,
    // Todo actions
    ToggleComplete,
    DeleteTodo,
    ToggleShowCompleted,
    IncreasePriority,
    DecreasePriority,
    // Drive sync
    DriveUpdated(String),
    SyncConflict {
        local_content: String,
        drive_content: String,
    },
    SyncStatusUpdate(SyncStatus),
    AcceptDriveVersion,
    KeepLocalVersion,
    // System
    SaveError(String),
    DismissError,
    Quit,
}
