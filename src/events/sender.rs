use tokio::sync::mpsc;

use super::event::{AppEvent, Event};

#[derive(Clone)]
pub struct EventSender {
    pub(crate) sender: mpsc::UnboundedSender<Event>,
}

impl EventSender {
    pub fn send(&self, event: AppEvent) {
        let _ = self.sender.send(Event::App(event));
    }
}
