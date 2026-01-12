# Zenith CLI Task Manager

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

**Zenith** is a futuristic, cyberpunk-styled terminal task manager built in Rust. It combines professional productivity tools with RPG-like gamification to make getting things done satisfying.

## 🚀 Features Implemented

- [x] **Cyberpunk TUI**: A beautiful, neon-styled interface powered by Ratatui.
- [x] **Kanban Board**: Visualize your workflow with interactive Todo / Doing / Done columns.
- [x] **Focus Mode**: Built-in Pomodoro timer to maintain flow state.
- [x] **RPG System**: Earn XP for every task, track your level, and gamify your life.
- [x] **Solid Architecture**: Built on `MVVM` pattern with `SQLite` persistence.

## 🎮 Controls

### Global
- `TAB`: Switch between **Dashboard**, **Kanban**, and **Focus** views.
- `q`: Quit application.

### Dashboard View
- `n`: Create a new task.
- `SPACE`: Toggle task status (Todo -> Doing -> Done).
- `d`: Delete selected task.
- `j` / `k`: Navigate the list.

### Kanban View
- `h` / `l`: Move focus between columns.
- `j` / `k`: Navigate tasks within a column.

### Focus View
- `t`: Start / Pause the Pomodoro timer.
- `r`: Reset the timer.

## 🛠️ Tech Stack

- **Language:** Rust 🦀
- **UI Engine:** Ratatui + Crossterm
- **Database:** SQLite (Rusqlite)
- **Error Handling:** Color-Eyre
- **Architecture:** MVVM + Domain Driven Design

## 📦 Installation

```bash
git clone https://github.com/miyaniakshar1234/zenith-cli
cd zenith-cli
cargo run --release
```
