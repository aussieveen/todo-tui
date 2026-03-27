# todo-tui

A terminal todo list manager following the [todo.txt](https://github.com/todotxt/todo.txt) format.

```
┌ Todos ───────────────────────────────────────────────────────────────────────────┐
│ Today                                                                            │
│  Fix authentication bug    @work +backend  created:2026-03-01  due:2026-03-08    │
│ This Week                                                                        │
│  Write release notes       @work +docs     created:2026-03-01                    │
│ This Sprint                                                                      │
│  Update dependencies       @work           created:2026-02-28                    │
│ Backlog                                                                          │
│  Review PR comments                        created:2026-03-07                    │
│ ✓ Old completed task       @work           done:2026-03-06                       │
└──────────────────────────────────────────────────────────────────────────────────┘
──────────────────────────── Help ──────────────────────────────────────────────────
 [↑↓] navigate  [Shift ↑↓] priority  [a] add  [e/↵] edit  [x/spc] complete
 [d] delete  [t] completed  [q/Esc] quit
```

## Features

- **todo.txt format** — todos are stored in `~/.todo/todo.txt`, a plain text file you can edit directly or sync with any tool that understands the format.
- **Priority system** — priorities A–E with clear urgency meanings (see below). Todos are sorted by priority, highest first, with unprioritised todos in a Backlog group at the bottom.
- **Contexts and projects** — tag todos with `@context` and `+project` in the description.
- **Due dates** — set a due date using `due:YYYY-MM-DD` in the description or via the add/edit popup.
- **Keyboard-driven** — no mouse required. Navigate with arrow keys, act with single keypresses.
- **Persistent** — todos are saved to disk immediately on every change.
- **Google Drive sync** — optional background sync keeps multiple machines in sync via a shared Drive file. Works offline; pushes when connectivity is restored.

## Priority Levels

Todos are grouped and colour-coded by priority. Priorities are entered as a single letter (uppercase or lowercase — lowercase is automatically converted).

| Priority | Group         | Colour  |
|----------|---------------|---------|
| `(A)`    | Today         | Red     |
| `(B)`    | This Week     | Yellow  |
| `(C)`    | This Sprint   | Green   |
| `(D)`    | This Month    | Blue    |
| `(E)`    | This Quarter  | Magenta |
| *(none)* | Backlog       | Default |

## Key Bindings

### Main view

| Key              | Action                                          |
|------------------|-------------------------------------------------|
| `↓`              | Move selection down                             |
| `↑`              | Move selection up (↑ from top clears selection) |
| `Shift ↑`        | Increase priority                               |
| `Shift ↓`        | Decrease priority                               |
| `a`              | Add todo                                        |
| `e` / `↵`        | Edit selected todo                              |
| `x` / `Space`    | Toggle complete                                 |
| `d`              | Delete selected todo                            |
| `t`              | Toggle showing completed todos                  |
| `q` / `Esc`      | Quit                                            |

### Add / Edit popup

| Key       | Action        |
|-----------|---------------|
| `Tab`     | Next field    |
| `Shift+Tab` | Previous field |
| `↵`       | Save          |
| `Esc`     | Cancel        |

### Sync conflict popup

Shown when both local and Drive have changed since the last sync.

| Key | Action                  |
|-----|-------------------------|
| `d` | Use Drive version       |
| `l` | Keep local version      |

## Google Drive Sync

todo-tui can sync your todos to Google Drive so multiple machines stay in sync automatically. The `todo.txt` file format is unchanged — Drive just stores the same plain text file.

### How it works

- On every save, the current file is pushed to Drive in the background.
- A background task polls Drive every 30 seconds (configurable). If Drive has newer content, it pulls automatically.
- If both sides changed since the last sync (e.g. you edited offline on two machines), a conflict popup appears so you can choose which version to keep. Both versions are saved to `~/.todo/conflicts/` for reference.
- Works offline — changes are buffered and pushed when connectivity is restored.

### Setup

**1. Create a Google Cloud project**

1. Go to [console.cloud.google.com](https://console.cloud.google.com)
2. Create a new project
3. Navigate to **APIs & Services → Library** and enable the **Google Drive API**
4. Navigate to **APIs & Services → Credentials**
5. Click **Create Credentials → OAuth 2.0 Client ID**
6. When asked what data you'll be accessing, choose **User data**
7. Fill in the OAuth consent screen (app name and your email are sufficient)
8. For scopes, add `https://www.googleapis.com/auth/drive.file` — this only grants access to files the app creates, not your entire Drive
9. Choose **Desktop app** as the application type
10. Download the JSON file and save it as `~/.todo/credentials.json`

**2. Enable sync in the config**

Copy the example config and enable sync:

```bash
cp config.example.toml ~/.todo/config.toml
```

Then open `~/.todo/config.toml` and set `enabled = true`:

```toml
[drive]
enabled = true
sync_interval_secs = 30
file_id = ""
credentials_path = "credentials.json"
```

`file_id` is populated automatically after the first successful sync. `credentials_path` is relative to `~/.todo/` unless an absolute path is given.

**3. Add yourself as a test user**

Because this app is not published through Google's verification process, you need to allow your own account to authorise it:

1. In Google Cloud Console, go to **APIs & Services → OAuth consent screen**
2. Scroll to **Test users** and click **Add users**
3. Add your Google account email and save

Alternatively, if you see a warning during authorisation, click **Advanced → Go to [app name] (unsafe)** to proceed. This is safe — it simply means the app hasn't gone through Google's public verification process.

**4. Authorise on first run**

On the first launch with sync enabled, the app will print an auth URL to the terminal before starting:

```
=== Google Drive authorisation required ===
Open this URL in your browser:

  https://accounts.google.com/o/oauth2/auth?...

Paste the authorisation code here: _
```

Visit the URL, grant access, paste the code back, and press Enter. The token is saved to `~/.todo/token.json` — subsequent launches are automatic.

### Files

| Path | Purpose |
|------|---------|
| `~/.todo/credentials.json` | OAuth2 client credentials (from Google Cloud Console) |
| `~/.todo/token.json` | Saved access/refresh token (created on first auth) |
| `~/.todo/config.toml` | Sync configuration |
| `~/.todo/last_sync.txt` | Last version synced to Drive (used for conflict detection) |
| `~/.todo/conflicts/` | Timestamped conflict logs |

## Installation

### Prebuilt Binaries (Recommended)

Prebuilt binaries are available for macOS and Linux.

1. Go to the [latest release](https://github.com/aussieveen/todo-tui/releases/latest) of this repository.
2. Download the archive for your operating system.
3. Extract the archive.
4. Move the binary into a directory on your `PATH`, for example:
   ```bash
   sudo mv todo-tui /usr/local/bin
   ```
5. Run `todo-tui` from the command line.

Todos are stored at `~/.todo/todo.txt` and the directory is created automatically on first run.

### Run From Source

If you have Rust installed:

```bash
git clone git@github.com:aussieveen/todo-tui.git
cd todo-tui
cargo run
```

### Supported Platforms

- macOS (Intel)
- Linux (x86_64)

### Notes

Ensure the binary is executable: `chmod +x todo-tui`

Make sure `/usr/local/bin` (or your chosen directory) is included in your `PATH`.

## License

Copyright (c) Simon McWhinnie <simon.mcwhinnie@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
