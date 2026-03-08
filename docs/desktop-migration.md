# Desktop Migration Notes

This repository has been migrated from a local Spring Boot runtime to a Tauri desktop runtime.

## What Changed

Old local shape:

- remote crawler
- local Spring Boot server
- browser frontend on a localhost port

Current shape:

- remote crawler
- local Tauri desktop app with Rust + Vue

## Data Migration

The desktop app preserves compatibility with:

- `memory.txt`
- `MGSList.txt`

On startup the app:

1. checks whether those files already exist in the Tauri app data directory
2. if not, scans legacy `GoNyaa-Server` style working directories
3. copies matching files into the new app data directory

## Runtime Configuration

The crawler base URL can be overridden with:

- `GONYAA_CRAWLER_BASE_URL`

If the variable is not set, the desktop app uses the built-in default crawler URL.

## Build and Release Flow

Development:

```bash
cd GoNyaa-Desktop
npm install
npm run tauri:dev
```

Validation:

```bash
cd GoNyaa-Desktop
npm run build
cd src-tauri
cargo check
```

Bundle:

```bash
cd GoNyaa-Desktop
npm run tauri:build
```

## Repository Status

`GoNyaa-Server` has been removed from the repository.
Legacy references only remain for one-time file migration compatibility.