use color_eyre::eyre::OptionExt;
use tokio::sync::mpsc;

use super::{event::Event, sender::EventSender, task::EventTask};

pub struct EventHandler {
    sender: EventSender,
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let sender = EventSender { sender: tx.clone() };
        let task = EventTask::new(tx);
        tokio::spawn(async { task.run().await });

        Self {
            sender,
            receiver: rx,
        }
    }

    pub fn sender(&self) -> EventSender {
        self.sender.clone()
    }

    pub async fn next(&mut self) -> color_eyre::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("event channel closed")
    }
}
