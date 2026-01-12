# Zenith CLI Task Manager v1.1.0 🚀

![Build Status](https://img.shields.io/github/actions/workflow/status/miyaniakshar1234/zenith-cli/release.yml?style=flat-square&label=Release%20Build)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/zenith-cli?style=flat-square)
![NPM](https://img.shields.io/npm/v/@miyaniakshar/zenith-cli?style=flat-square)

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

### Method 1: Direct Download (No Rust Required) 🌟
The easiest way to get started. No coding knowledge needed.

1.  Go to the **[Releases Page](https://github.com/miyaniakshar1234/zenith-cli/releases)**.
2.  Download the file for your system:
    - **Windows**: `zenith-cli-windows-amd64.exe`
    - **Mac**: `zenith-cli-macos-amd64`
    - **Linux**: `zenith-cli-linux-amd64`
3.  **Run it!**
    - Windows: Double-click the `.exe` or run from PowerShell.
    - Mac/Linux: Open terminal, `chmod +x zenith-cli`, then `./zenith-cli`.

### Method 2: Via NPM (Universal / Restricted Networks) 🌐
If you have Node.js installed (common in schools/bootcamps) and cannot access GitHub binary downloads:

```bash
npx @miyaniakshar/zenith-cli
```
*This works instantly without administrator privileges and bypasses GitHub download blocks.*

### Method 3: Via Terminal (For Developers)
If you have Rust/Cargo installed:

```bash
cargo install zenith-cli
```
*Or build from source:*
```bash
cargo install --git https://github.com/miyaniakshar1234/zenith-cli
```

## 🚀 Running the Application
Once installed via Cargo or NPM, simply type:
```bash
zenith-cli
```
If you downloaded the binary, navigate to your download folder and run it directly (`.\zenith-cli.exe`).

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
