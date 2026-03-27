#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum KeyContext {
    /// Normal list view — navigating todos.
    Main,
    /// Add/edit popup is open.
    Popup,
    /// Error overlay is active.
    ErrorPopup,
    /// Sync conflict resolution overlay is active.
    SyncConflict,
    /// Always-active bindings.
    Global,
}
