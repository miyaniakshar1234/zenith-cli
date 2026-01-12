# Zenith CLI Task Manager v1.0.0 🚀

![Build Status](https://img.shields.io/github/actions/workflow/status/miyaniakshar1234/zenith-cli/ci.yml?style=flat-square&label=Build)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Rust](https://img.shields.io/badge/Rust-v1.8%2B-orange?style=flat-square)

```text
███████╗███████╗███╗   ██╗██╗████████╗██╗  ██╗
╚══███╔╝██╔════╝████╗  ██║██║╚══██╔══╝██║  ██║
  ███╔╝ █████╗  ██╔██╗ ██║██║   ██║   ███████║
 ███╔╝  ██╔══╝  ██║╚██╗██║██║   ██║   ██╔══██║
███████╗███████╗██║ ╚████║██║   ██║   ██║  ██║
╚══════╝╚══════╝╚═╝  ╚═══╝╚═╝   ╚═╝   ╚═╝  ╚═╝
   >> HIGH-PERFORMANCE CLI TASK MANAGER <<
```

**Zenith** is a futuristic, industrial-grade terminal task manager built in Rust. It combines professional productivity tools with RPG-like gamification to make getting things done satisfying.

## 🚀 Features

- [x] **Cyberpunk/Nord UI**: A beautiful, themeable TUI with sidebar navigation.
- [x] **Kanban Board**: Visualize your workflow with interactive Todo / Doing / Done columns.
- [x] **Focus Mode**: Built-in Pomodoro timer to maintain flow state.
- [x] **Analytics**: Visual charts tracking your weekly productivity velocity.
- [x] **RPG System**: Earn XP for every task, track your level, and gamify your life.
- [x] **Industrial Editor**: Multi-line task descriptions with Vim-like cursor control.
- [x] **Search & Filter**: Instant `/` search filtering.

## 🎮 Controls

### Global
- `TAB`: Switch between **Dashboard**, **Kanban**, **Focus**, and **Analytics** views.
- `q`: Quit application.

### Dashboard View
- `n`: Create a new task (opens Editor).
- `e`: Edit selected task.
- `d`: Delete selected task.
- `SPACE`: Toggle task status (Todo -> Doing -> Done).
- `ENTER`: **Inspect Task** (View details & full description).
- `/`: **Search Mode** (Type to filter tasks).
- `j` / `k`: Navigate the list.

### Kanban View
- `h` / `l`: Move focus between columns.
- `j` / `k`: Navigate tasks within a column.

### Focus View
- `t`: Start / Pause the Pomodoro timer.
- `r`: Reset the timer.

## 🛠️ Tech Stack

- **Language:** Rust 🦀
- **UI Engine:** Ratatui + Crossterm + Tui-Textarea
- **Database:** SQLite (Rusqlite)
- **Error Handling:** Color-Eyre
- **Architecture:** MVVM + Domain Driven Design

## 📦 Installation

```bash
git clone https://github.com/miyaniakshar1234/zenith-cli
cd zenith-cli
cargo run --release
```
