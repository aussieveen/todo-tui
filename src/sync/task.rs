use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Utc};
use tokio::{sync::mpsc, task::JoinHandle, time};

use crate::events::{event::AppEvent, sender::EventSender};

use super::{SyncStatus, drive::DriveClient};

pub struct SyncTask;

impl SyncTask {
    /// Spawn the background sync task.
    ///
    /// `push_rx` receives serialised todo.txt content from the main loop whenever the
    /// user saves. The task immediately uploads it to Drive.
    ///
    /// `event_sender` is used to send `AppEvent` variants back to the main loop.
    pub fn spawn(
        mut drive: DriveClient,
        todo_dir: PathBuf,
        file_id: String,
        event_sender: EventSender,
        mut push_rx: mpsc::UnboundedReceiver<String>,
        interval: Duration,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let base_path = todo_dir.join("last_sync.txt");
            let todo_path = todo_dir.join("todo.txt");
            let mut poll_interval = time::interval(interval);
            poll_interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

            // Track the Drive modifiedTime we last saw so we don't re-pull unchanged content.
            let mut last_known_drive_time: Option<DateTime<Utc>> = None;
            // Buffer a pending push if we're offline.
            let mut pending_push: Option<String> = None;

            loop {
                tokio::select! {
                    // Push triggered by user save
                    Some(content) = push_rx.recv() => {
                        pending_push = Some(content.clone());
                        match push(&mut drive, &file_id, &content, &base_path).await {
                            Ok(new_time) => {
                                pending_push = None;
                                last_known_drive_time = Some(new_time);
                                event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Synced(new_time)));
                            }
                            Err(_) => {
                                event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Offline));
                            }
                        }
                    }

                    // Periodic poll
                    _ = poll_interval.tick() => {
                        // Retry pending push first
                        if let Some(content) = pending_push.clone() {
                            match push(&mut drive, &file_id, &content, &base_path).await {
                                Ok(new_time) => {
                                    pending_push = None;
                                    last_known_drive_time = Some(new_time);
                                    event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Synced(new_time)));
                                }
                                Err(_) => {
                                    event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Offline));
                                    continue;
                                }
                            }
                        }

                        event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Syncing));

                        match poll(&mut drive, &file_id, &base_path, &todo_path, last_known_drive_time).await {
                            Ok(PollResult::NoChange(t)) => {
                                last_known_drive_time = Some(t);
                                event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Synced(t)));
                            }
                            Ok(PollResult::Updated { drive_content, drive_time }) => {
                                last_known_drive_time = Some(drive_time);
                                event_sender.send(AppEvent::DriveUpdated(drive_content));
                                event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Synced(drive_time)));
                            }
                            Ok(PollResult::Conflict { local_content, drive_content, drive_time }) => {
                                last_known_drive_time = Some(drive_time);
                                event_sender.send(AppEvent::SyncConflict { local_content, drive_content });
                            }
                            Ok(PollResult::LocalAhead) => {
                                // Local has changes; push was not triggered (e.g. file edited externally).
                                // Nothing to do — the next user save will push.
                            }
                            Err(_) => {
                                event_sender.send(AppEvent::SyncStatusUpdate(SyncStatus::Offline));
                            }
                        }
                    }
                }
            }
        })
    }
}

enum PollResult {
    NoChange(DateTime<Utc>),
    Updated {
        drive_content: String,
        drive_time: DateTime<Utc>,
    },
    Conflict {
        local_content: String,
        drive_content: String,
        drive_time: DateTime<Utc>,
    },
    LocalAhead,
}

async fn poll(
    drive: &mut DriveClient,
    file_id: &str,
    base_path: &std::path::Path,
    todo_path: &std::path::Path,
    last_known_drive_time: Option<DateTime<Utc>>,
) -> Result<PollResult, String> {
    let drive_time = drive.get_modified_time(file_id).await?;

    // If Drive hasn't changed since our last check, nothing to do.
    if last_known_drive_time.is_some_and(|t| t >= drive_time) {
        return Ok(PollResult::NoChange(drive_time));
    }

    let drive_content = drive.download(file_id).await?;
    let base_content = std::fs::read_to_string(base_path).unwrap_or_default();
    let local_content = std::fs::read_to_string(todo_path).unwrap_or_default();

    let drive_changed = drive_content.trim() != base_content.trim();
    let local_changed = local_content.trim() != base_content.trim();

    match (local_changed, drive_changed) {
        (false, false) => Ok(PollResult::NoChange(drive_time)),
        (false, true) => Ok(PollResult::Updated {
            drive_content,
            drive_time,
        }),
        (true, false) => Ok(PollResult::LocalAhead),
        (true, true) => Ok(PollResult::Conflict {
            local_content,
            drive_content,
            drive_time,
        }),
    }
}

async fn push(
    drive: &mut DriveClient,
    file_id: &str,
    content: &str,
    base_path: &std::path::Path,
) -> Result<DateTime<Utc>, String> {
    drive.upload(file_id, content).await?;
    // Update the base so future conflict detection knows both sides are in sync.
    std::fs::write(base_path, content).map_err(|e| e.to_string())?;
    // Return the new modifiedTime from Drive so our poll cursor is up to date.
    drive.get_modified_time(file_id).await
}
