use crossterm::event::Event as CrosstermEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Crossterm(CrosstermEvent),
    App(AppEvent),
}

#[derive(Debug, Clone, PartialEq)]
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
    // System
    SaveError(String),
    DismissError,
    Quit,
}
