use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::events::event::AppEvent;

use super::{key_context::KeyContext::*, key_event_map::KeyEventMap};

pub fn register_bindings(map: &mut KeyEventMap) {
    // --- Global ---
    map.add_static(
        Global,
        KeyCode::Char('q'),
        KeyModifiers::NONE,
        AppEvent::Quit,
    );
    map.add_static(Global, KeyCode::Esc, KeyModifiers::NONE, AppEvent::Quit);

    // --- Error popup ---
    map.add_static(
        ErrorPopup,
        KeyCode::Char('d'),
        KeyModifiers::NONE,
        AppEvent::DismissError,
    );
    map.add_static(
        ErrorPopup,
        KeyCode::Esc,
        KeyModifiers::NONE,
        AppEvent::DismissError,
    );

    // --- Main view ---
    map.add_static(Main, KeyCode::Up, KeyModifiers::NONE, AppEvent::MoveUp);
    map.add_static(Main, KeyCode::Down, KeyModifiers::NONE, AppEvent::MoveDown);
    map.add_static(
        Main,
        KeyCode::Up,
        KeyModifiers::SHIFT,
        AppEvent::IncreasePriority,
    );
    map.add_static(
        Main,
        KeyCode::Down,
        KeyModifiers::SHIFT,
        AppEvent::DecreasePriority,
    );
    map.add_static(
        Main,
        KeyCode::Char('n'),
        KeyModifiers::NONE,
        AppEvent::OpenAddPopup,
    );
    map.add_static(
        Main,
        KeyCode::Char('e'),
        KeyModifiers::NONE,
        AppEvent::OpenEditPopup,
    );
    map.add_static(
        Main,
        KeyCode::Enter,
        KeyModifiers::NONE,
        AppEvent::OpenEditPopup,
    );
    map.add_static(
        Main,
        KeyCode::Char('x'),
        KeyModifiers::NONE,
        AppEvent::ToggleComplete,
    );
    map.add_static(
        Main,
        KeyCode::Char(' '),
        KeyModifiers::NONE,
        AppEvent::ToggleComplete,
    );
    map.add_static(
        Main,
        KeyCode::Char('d'),
        KeyModifiers::NONE,
        AppEvent::DeleteTodo,
    );
    map.add_static(
        Main,
        KeyCode::Char('t'),
        KeyModifiers::NONE,
        AppEvent::ToggleShowCompleted,
    );
    // --- Popup ---
    map.add_static(
        Popup,
        KeyCode::Enter,
        KeyModifiers::NONE,
        AppEvent::SubmitPopup,
    );
    map.add_static(
        Popup,
        KeyCode::Esc,
        KeyModifiers::NONE,
        AppEvent::CancelPopup,
    );
    map.add_static(Popup, KeyCode::Tab, KeyModifiers::NONE, AppEvent::NextField);
    map.add_static(
        Popup,
        KeyCode::BackTab,
        KeyModifiers::SHIFT,
        AppEvent::PrevField,
    );
    map.add_static(
        Popup,
        KeyCode::Backspace,
        KeyModifiers::NONE,
        AppEvent::RemoveChar,
    );
    map.add_dynamic(Popup, popup_char_input);
}

fn popup_char_input(key: KeyEvent) -> Option<AppEvent> {
    if let KeyCode::Char(c) = key.code
        && (key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT) {
            return Some(AppEvent::AddChar(c));
        }
    None
}
