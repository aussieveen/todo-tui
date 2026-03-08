use std::time::Duration;

use futures::StreamExt;
use tokio::sync::mpsc;

use super::event::Event;

const TICK_FPS: f64 = 30.0;

pub struct EventTask {
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    pub fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    pub async fn run(self) -> color_eyre::Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next();

            tokio::select! {
                _ = self.sender.closed() => break,
                _ = tick_delay => {
                    self.send(Event::Tick);
                }
                Some(Ok(evt)) = crossterm_event => {
                    self.send(Event::Crossterm(evt));
                }
            }
        }

        Ok(())
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
