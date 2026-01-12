# Contributing to Zenith CLI

First off, thank you for considering contributing to **Zenith**! It's people like you that make the open-source community such an amazing place.

## 🛠️ Project Architecture
Zenith is built on the **MVVM** pattern using Rust.
- **`src/app.rs`**: Main application state and logic.
- **`src/ui/`**: All rendering logic (Ratatui widgets).
- **`src/db/`**: SQLite persistence layer.

## 🤝 How to Contribute

### 1. Fork & Clone
```bash
git clone https://github.com/miyaniakshar1234/zenith-cli
cd zenith-cli
```

### 2. Create a Branch
```bash
git checkout -b feature/amazing-feature
```

### 3. Make Changes & Test
```bash
cargo run
cargo test
cargo clippy  # Ensure zero warnings!
```

### 4. Commit & Push
Please use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat: add new widget`
- `fix: resolve crash on startup`
- `docs: update readme`

```bash
git commit -m "feat: add amazing feature"
git push origin feature/amazing-feature
```

### 5. Open a Pull Request
Go to GitHub and open a PR against `main`.

## 🎨 Style Guide
- Use `cargo fmt` before committing.
- Keep UI logic in `src/ui/` and business logic in `src/app.rs` or `src/db/`.
- Respect the "Horizon" theme aesthetic (clean, high contrast).

## 👤 Author
**Miyani Akshar**
- GitHub: [@miyaniakshar1234](https://github.com/miyaniakshar1234)
