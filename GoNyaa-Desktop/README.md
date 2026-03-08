# GoNyaa-Desktop

Tauri desktop client for GoNyaa.

## Stack

- Tauri 2
- Rust
- Vue 3
- Vite
- Element Plus

## Commands

Install dependencies:

```bash
npm install
```

Run the desktop app in development:

```bash
npm run tauri:dev
```

Run a frontend build only:

```bash
npm run build
```

Check the Rust backend:

```bash
cd src-tauri
cargo check
```

Build a desktop bundle:

```bash
npm run tauri:build
```

## Runtime Notes

- Viewed state is stored in `memory.txt`
- MGStage prefixes are stored in `MGSList.txt`
- Both files live under the Tauri app data directory
- Legacy files are migrated automatically on first launch when possible
- The crawler base URL can be overridden with `GONYAA_CRAWLER_BASE_URL`

## Current Scope

This desktop app replaces the old local Spring Boot server.
The crawler remains a separate service and is intentionally not migrated in this phase.