use crate::{
    app::App,
    events::event::AppEvent,
    persistence::Persister,
    state::{
        app::{AppFocus, ConflictState, PopupState},
        todo::Todo,
    },
};

pub fn handle_event(app: &mut App, event: AppEvent) {
    match event {
        AppEvent::MoveUp => app.state.move_up(),
        AppEvent::MoveDown => app.state.move_down(),

        AppEvent::OpenAddPopup => {
            app.state.popup = PopupState::for_add();
            app.state.focus = AppFocus::Popup;
        }

        AppEvent::OpenEditPopup => {
            if let Some(idx) = app.state.selected_todo_index()
                && let Some(todo) = app.state.todos.get(idx)
            {
                app.state.popup = PopupState::for_edit(idx, todo);
                app.state.focus = AppFocus::Popup;
            }
        }

        AppEvent::NextField => {
            app.state.popup.field = app.state.popup.field.next();
        }

        AppEvent::PrevField => {
            app.state.popup.field = app.state.popup.field.prev();
        }

        AppEvent::AddChar(c) => {
            let c = if app.state.popup.field == crate::state::app::PopupField::Priority {
                c.to_ascii_uppercase()
            } else {
                c
            };
            app.state.popup.active_field_mut().push(c);
        }

        AppEvent::RemoveChar => {
            app.state.popup.active_field_mut().pop();
        }

        AppEvent::SubmitPopup => {
            let desc = app.state.popup.full_description();
            if desc.trim().is_empty() {
                app.state.focus = AppFocus::Main;
                return;
            }

            let priority = app.state.popup.parsed_priority();
            let todo = Todo::new(desc, priority);

            match app.state.popup.edit_index {
                Some(idx) => {
                    if let Some(existing) = app.state.todos.get_mut(idx) {
                        existing.description = todo.description;
                        existing.priority = todo.priority;
                        existing.contexts = todo.contexts;
                        existing.projects = todo.projects;
                    }
                }
                None => {
                    app.state.todos.push(todo);
                    let last = app.state.selectable_count().saturating_sub(1);
                    app.state.selected = Some(last);
                    app.state.scroll_to_reveal();
                }
            }

            app.state.focus = AppFocus::Main;
            app.save();
        }

        AppEvent::CancelPopup => {
            app.state.focus = AppFocus::Main;
        }

        AppEvent::ToggleComplete => {
            if let Some(idx) = app.state.selected_todo_index() {
                if let Some(todo) = app.state.todos.get_mut(idx) {
                    todo.done = !todo.done;
                    if todo.done {
                        todo.completion_date = Some(chrono::Local::now().date_naive());
                    } else {
                        todo.completion_date = None;
                    }
                }
                app.state.clamp_selection();
                app.save();
            }
        }

        AppEvent::DeleteTodo => {
            if let Some(idx) = app.state.selected_todo_index() {
                app.state.todos.remove(idx);
                app.state.clamp_selection();
                app.save();
            }
        }

        AppEvent::ToggleShowCompleted => {
            app.state.show_completed = !app.state.show_completed;
            app.state.scroll_offset = 0;
            app.state.clamp_selection();
        }

        AppEvent::IncreasePriority => {
            if let Some(idx) = app.state.selected_todo_index() {
                if let Some(todo) = app.state.todos.get_mut(idx) {
                    todo.priority = increase_priority(todo.priority);
                }
                // Follow the item to its new sorted position
                app.state.selected = app
                    .state
                    .visible_todos()
                    .iter()
                    .position(|(i, _)| *i == idx);
                app.state.scroll_to_reveal();
                app.save();
            }
        }

        AppEvent::DecreasePriority => {
            if let Some(idx) = app.state.selected_todo_index() {
                if let Some(todo) = app.state.todos.get_mut(idx) {
                    todo.priority = decrease_priority(todo.priority);
                }
                app.state.selected = app
                    .state
                    .visible_todos()
                    .iter()
                    .position(|(i, _)| *i == idx);
                app.state.scroll_to_reveal();
                app.save();
            }
        }

        AppEvent::DriveUpdated(raw) => {
            app.state.todos = Persister::parse_content(&raw);
            if let Err(e) = app.persister.write_raw(&raw) {
                app.event_sender.send(AppEvent::SaveError(e.to_string()));
            }
            let _ = app.persister.save_base(&raw);
        }

        AppEvent::SyncConflict {
            local_content,
            drive_content,
        } => {
            let _ = app.persister.log_conflict(&local_content, &drive_content);
            app.state.conflict = Some(ConflictState {
                local_content,
                drive_content,
            });
            app.state.focus = AppFocus::SyncConflict;
        }

        AppEvent::AcceptDriveVersion => {
            if let Some(conflict) = app.state.conflict.take() {
                app.state.todos = Persister::parse_content(&conflict.drive_content);
                if let Err(e) = app.persister.write_raw(&conflict.drive_content) {
                    app.event_sender.send(AppEvent::SaveError(e.to_string()));
                }
                let _ = app.persister.save_base(&conflict.drive_content);
            }
            app.state.focus = AppFocus::Main;
        }

        AppEvent::KeepLocalVersion => {
            if let Some(conflict) = app.state.conflict.take() {
                let _ = app.persister.save_base(&conflict.local_content);
                if let Some(tx) = &app.push_tx {
                    let _ = tx.send(conflict.local_content);
                }
            }
            app.state.focus = AppFocus::Main;
        }

        AppEvent::SyncStatusUpdate(status) => {
            app.state.sync_status = status;
        }

        AppEvent::SaveError(msg) => {
            app.state.error = Some(msg);
            app.state.focus = AppFocus::ErrorPopup;
        }

        AppEvent::DismissError => {
            app.state.error = None;
            app.state.focus = AppFocus::Main;
        }

        AppEvent::Quit => {
            app.running = false;
        }
    }
}

fn increase_priority(p: Option<char>) -> Option<char> {
    match p {
        Some('A') => Some('A'),
        Some(c @ 'B'..='E') => Some((c as u8 - 1) as char),
        _ => Some('E'),
    }
}

fn decrease_priority(p: Option<char>) -> Option<char> {
    match p {
        Some('E') => None,
        Some(c @ 'A'..='D') => Some((c as u8 + 1) as char),
        _ => None,
    }
}
