# Zenith CLI Task Manager v1.1.0 🚀

![Build Status](https://img.shields.io/github/actions/workflow/status/miyaniakshar1234/zenith-cli/release.yml?style=flat-square&label=Build)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/zenith-cli?style=flat-square)

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

## 📦 Installation via Terminal

### Method 1: The "Hacker" Way (Recommended)
You can install Zenith directly from the source code with one command. This works on Windows, Mac, and Linux.

```bash
cargo install --git https://github.com/miyaniakshar1234/zenith-cli
```
*Prerequisite: You need to have Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)*

### Method 2: From Crates.io (If Published)
Once published to the official registry, you can simply run:

```bash
cargo install zenith-cli
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

## 👨‍💻 Created By

**Miyani Akshar**
*Systems Engineer & Open Source Enthusiast*

- **GitHub**: [@miyaniakshar1234](https://github.com/miyaniakshar1234)
- **Crates.io**: [zenith-cli](https://crates.io/crates/zenith-cli)

Built with ❤️ in **Rust** to help developers stay in the flow.
