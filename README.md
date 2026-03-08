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
 [↑↓] navigate  [Shift ↑↓] priority  [n] new  [e/↵] edit  [x/spc] complete
 [d] delete  [t] completed  [q/Esc] quit
```

## Features

- **todo.txt format** — todos are stored in `~/.todo/todo.txt`, a plain text file you can edit directly or sync with any tool that understands the format.
- **Priority system** — priorities A–E with clear urgency meanings (see below). Todos are sorted by priority, highest first, with unprioritised todos in a Backlog group at the bottom.
- **Contexts and projects** — tag todos with `@context` and `+project` in the description.
- **Due dates** — set a due date using `due:YYYY-MM-DD` in the description or via the add/edit popup.
- **Keyboard-driven** — no mouse required. Navigate with arrow keys, act with single keypresses.
- **Persistent** — todos are saved to disk immediately on every change.

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
| `n`              | Add new todo                                    |
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
