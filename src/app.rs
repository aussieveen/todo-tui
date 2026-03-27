use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::{
    events::{
        event::{AppEvent, Event},
        handler::EventHandler,
        sender::EventSender,
        tools::todo as todo_handler,
    },
    input::{key_bindings::register_bindings, key_context::KeyContext, key_event_map::KeyEventMap},
    persistence::Persister,
    state::app::{AppFocus, AppState},
    sync::{SyncParams, SyncStatus, task::SyncTask},
    ui::{layout, widgets},
};

pub struct App {
    pub(crate) running: bool,
    pub(crate) state: AppState,
    event_handler: EventHandler,
    pub(crate) event_sender: EventSender,
    key_event_map: KeyEventMap,
    pub(crate) persister: Persister,
    /// Channel to send serialised content to the background sync task for upload.
    pub(crate) push_tx: Option<mpsc::UnboundedSender<String>>,
    /// Sync task parameters, consumed once on the first call to run().
    sync_params: Option<SyncParams>,
}

impl App {
    pub fn new(persister: Persister, sync_params: Option<SyncParams>) -> Self {
        let event_handler = EventHandler::new();
        let event_sender = event_handler.sender();
        Self {
            running: true,
            state: AppState::new(),
            event_handler,
            event_sender,
            key_event_map: KeyEventMap::default(),
            persister,
            push_tx: None,
            sync_params,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        register_bindings(&mut self.key_event_map);

        // Spawn the background sync task now that we have event_sender.
        if let Some(params) = self.sync_params.take() {
            let (push_tx, push_rx) = mpsc::unbounded_channel::<String>();
            self.push_tx = Some(push_tx);
            self.state.sync_status = SyncStatus::Idle;
            SyncTask::spawn(
                params.drive,
                params.todo_dir,
                params.file_id,
                self.event_sender.clone(),
                push_rx,
                params.interval,
            );
        }

        // Load todos from disk on startup
        match self.persister.load() {
            Ok(todos) => self.state.todos = todos,
            Err(e) => {
                self.state.error = Some(e.to_string());
                self.state.focus = AppFocus::ErrorPopup;
            }
        }

        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            match self.event_handler.next().await? {
                Event::Tick => {}
                Event::Crossterm(CrosstermEvent::Key(key)) if key.kind == KeyEventKind::Press => {
                    self.handle_key(key);
                }
                Event::Crossterm(_) => {}
                Event::App(event) => todo_handler::handle_event(&mut self, event),
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let areas = layout::main(frame.area());

        let pending = self.state.todos.iter().filter(|t| !t.done).count();
        let completed = self.state.todos.iter().filter(|t| t.done).count();

        widgets::header::render(frame, areas.header, pending, completed);
        self.state.viewport_height = areas.content.height.saturating_sub(2);
        widgets::content::render(frame, areas.content, &self.state);
        widgets::footer::render(frame, areas.footer, &self.state);

        if self.state.focus == AppFocus::Popup {
            widgets::popup::render(frame, frame.area(), &self.state.popup);
        }

        if let Some(msg) = &self.state.error {
            widgets::popup::render_error(frame, frame.area(), &msg.clone());
        }

        if self.state.focus == AppFocus::SyncConflict {
            if let Some(conflict) = &self.state.conflict {
                widgets::conflict::render(frame, frame.area(), conflict);
            }
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        for context in self.context_stack() {
            if let Some(event) = self.key_event_map.resolve(context, key) {
                self.event_sender.send(event);
                return;
            }
        }
    }

    fn context_stack(&self) -> Vec<KeyContext> {
        let mut stack = Vec::new();

        match self.state.focus {
            AppFocus::ErrorPopup => {
                stack.push(KeyContext::ErrorPopup);
            }
            AppFocus::Popup => {
                stack.push(KeyContext::Popup);
            }
            AppFocus::SyncConflict => {
                stack.push(KeyContext::SyncConflict);
            }
            AppFocus::Main => {
                stack.push(KeyContext::Main);
            }
        }

        stack.push(KeyContext::Global);
        stack
    }

    /// Persist current todos locally and trigger a Drive push if sync is enabled.
    pub(crate) fn save(&self) {
        if let Err(e) = self.persister.save(&self.state.todos) {
            self.event_sender.send(AppEvent::SaveError(e.to_string()));
            return;
        }
        if let Some(tx) = &self.push_tx {
            let content = Persister::serialise(&self.state.todos);
            let _ = tx.send(content);
        }
    }
}
