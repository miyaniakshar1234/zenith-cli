# Zenith CLI Task Manager v1.1.0 🚀

![Build Status](https://img.shields.io/github/actions/workflow/status/miyaniakshar1234/zenith-cli/release.yml?style=flat-square&label=Release%20Build)
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
- [x] **Theme Switcher**: Press `T` to toggle themes (Horizon, Nebula, Nord, Cyberpunk).

## 📦 Installation

### Option 1: Direct Download (Recommended)
Go to the **[Releases Page](https://github.com/miyaniakshar1234/zenith-cli/releases)** and download the binary for your system (Windows, Linux, or macOS).
- **Windows**: Download `zenith-cli-windows-amd64.exe`
- **Linux**: Download `zenith-cli-linux-amd64`
- **Mac**: Download `zenith-cli-macos-amd64`

### Option 2: Build from Source
If you have Rust installed:
```bash
git clone https://github.com/miyaniakshar1234/zenith-cli
cd zenith-cli
cargo install --path .
```

## 🎮 Controls

### Global
- `TAB`: Switch Views (Dashboard -> Kanban -> Focus -> Analytics).
- `?`: Toggle **Command Palette** (Help).
- `T`: Switch Theme.
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
