# AGENTS.md — mtranslate

## What this is
Myridia's online translation service — a Rust (axum) backend that serves translations, backed by a large set of MySQL translation tables (one per language pair).

## Stack
- Rust (axum, tokio, serde, mysql, deep-translator)
- MySQL/MariaDB
- Docker (dockers/)

## Build
```bash
cargo build
```

## Run
```bash
cargo run   (backend API serving translation endpoints)
```

## Structure
- `src/` — Rust source (`main.rs`, `lib.rs`)
- `Cargo.toml` — dependencies (axum, mysql, deep-translator, sha2, uuid...)
- `wordlists/`, `test/`, `pages/`, `dockers/` — supporting assets
- `todo.txt` / `ask.sh` / `rustfmt.toml` — dev helpers

## Conventions
- No comments in code unless asked.
- Verify: `cargo check && cargo build`.
- Formatting via rustfmt (`rustfmt.toml`).
