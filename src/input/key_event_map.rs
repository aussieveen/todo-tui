use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::events::event::AppEvent;

use super::key_context::KeyContext;

type Key = (KeyCode, KeyModifiers);
type KeyHandler = fn(KeyEvent) -> Option<AppEvent>;

#[derive(Default)]
pub struct KeyEventMap {
    static_events: HashMap<(KeyContext, Key), AppEvent>,
    dynamic_events: HashMap<KeyContext, KeyHandler>,
}

impl KeyEventMap {
    pub fn add_static(
        &mut self,
        context: KeyContext,
        key_code: KeyCode,
        modifiers: KeyModifiers,
        event: AppEvent,
    ) {
        self.static_events
            .insert((context, (key_code, modifiers)), event);
    }

    pub fn add_dynamic(&mut self, context: KeyContext, handler: KeyHandler) {
        self.dynamic_events.insert(context, handler);
    }

    pub fn resolve(&self, context: KeyContext, key: KeyEvent) -> Option<AppEvent> {
        let event = self
            .static_events
            .get(&(context.clone(), (key.code, key.modifiers)))
            .cloned();

        if event.is_some() {
            return event;
        }

        self.dynamic_events
            .get(&context)
            .and_then(|handler| handler(key))
    }
}
