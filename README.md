# Zenith CLI Task Manager v1.1.0 🚀

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

- [x] **Horizon UI**: A deep-space aesthetic with Master-Detail dashboard layout.
- [x] **Task Wizard**: Structured form for Title, Description, Priority, and Rewards.
- [x] **Smart Parsing**: Type `!h` in title for High Priority, or `> Reward: 50` in description.
- [x] **Kanban Board**: Interactive workflow management.
- [x] **Focus Mode**: Distraction-free Pomodoro timer.
- [x] **Analytics**: Visual productivity velocity charts.
- [x] **RPG System**: Level up as you complete tasks.

## 🎮 Controls

### Global
- `TAB`: Switch Views (Dashboard -> Kanban -> Focus -> Analytics).
- `?`: Toggle **Command Palette** (Help).
- `q` / `Ctrl+C`: Quit application.

### Dashboard View
- `n`: **New Task Wizard**.
- `e`: **Edit Task**.
- `d`: Delete Task.
- `SPACE`: Toggle Status.
- `/`: Search.
- `j` / `k`: Navigate.

### Task Wizard (Edit Mode)
- `TAB`: Next Field.
- `Shift+TAB`: Previous Field.
- `Enter`: Save Task.
- `Esc`: Cancel.
- `Left`/`Right`: Change Priority (in Priority field).

### Focus View
- `t`: Start / Pause Timer.
- `r`: Reset Timer.

## 🛠️ Tech Stack

- **Core**: Rust 🦀 + SQLite
- **UI**: Ratatui + Crossterm
- **Input**: Tui-Textarea
- **Architecture**: MVVM + Domain Driven Design

## 📦 Installation

```bash
git clone https://github.com/miyaniakshar1234/zenith-cli
cd zenith-cli
cargo run --release
```
