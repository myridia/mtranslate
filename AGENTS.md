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
- `src/` — Rust source (`main.rs`, `lib.rs`, `translate.rs`, `html_translate.rs`, `help.rs`, `test.rs`, `config.rs`)
- `Cargo.toml` — dependencies (axum, mysql, deep-translator, sha2, uuid...)
- `wordlists/`, `test/`, `pages/`, `dockers/` — supporting assets
- `dockers/migrate_translit.sh` — idempotent DB migration: adds `translit` column to all 104 language tables (`ADD COLUMN IF NOT EXISTS`)
- `todo.txt` / `ask.sh` / `rustfmt.toml` — dev helpers

## Conventions
- No comments in code unless asked.
- Verify: `cargo check && cargo build`.
- Formatting via rustfmt (`rustfmt.toml`).
- DB schema created/managed via https://textmaker.myridia.com; per-table `CREATE TABLE` + maintenance queries in `README.md`.

## Migrations
Run `dockers/migrate_translit.sh` with the `db` mariadb container up (`./ask.sh` task 1). Idempotent; re-run-safe. Container name is `db` (from `dockers/docker-compose.yml`), DB/user/pass `dbsql1`/`dbsql1`/`passpass`.
