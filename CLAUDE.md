# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GoNyaa is a Tauri v2 desktop application for browsing video metadata from Nyaa (torrent indexer). It consists of:

1. **GoNyaa-Client/** - Tauri desktop app (Rust backend + Vue 3 frontend)
2. **GoNyaa-Crawler/** - Python Flask scraper (external dependency, runs separately)
3. **GoNyaa-Server/** - Legacy Java Spring Boot backend (deprecated, kept for reference)

The Rust backend communicates with the Python crawler via HTTP and manages local state (viewed items, MGStage prefixes, caching).

## Build Commands

### GoNyaa-Client (Tauri App)

```bash
cd GoNyaa-Client

# Development (starts Vue dev server + Tauri)
npm run tauri dev

# Build production app
npm run tauri build

# Vue only (frontend dev server on port 8080)
npm run serve

# Build Vue frontend only
npm run build

# Lint
npm run lint
```

### Python Crawler (External Dependency)

```bash
cd GoNyaa-Crawler
pip install flask beautifulsoup4 requests lxml
python Nyaa_Get.py  # Starts on port 5001
```

### Legacy Java Backend (Deprecated)

```bash
cd GoNyaa-Server
mvn clean package
java -jar target/GoNyaa-1.3.1.jar
```

## Architecture

### Data Flow

```
User -> Vue Frontend (Tauri WebView)
  -> Rust Backend (Tauri Commands)
    -> Python Flask (port 5001) [scraping]
      -> sukebei.nyaa.si
```

Local state (memory.txt, MGSList.txt) is managed by the Rust backend.

### Rust Backend Structure (src-tauri/src/)

| File | Purpose |
|------|---------|
| `lib.rs` | Entry point, plugin initialization, command registration |
| `commands.rs` | Tauri command handlers (10 commands) |
| `api.rs` | HTTP client to Python backend, response parsing |
| `state.rs` | `AppState` struct with Mutex-protected shared state |
| `models.rs` | Data structures (`VideoInfoItemVo`, `AppConfig`) |
| `file_ops.rs` | File I/O for `application.json`, `memory.txt`, `MGSList.txt` |
| `utils.rs` | Type classification, link generation, sorting |

### Commands (commands.rs)

| Command | Description |
|---------|-------------|
| `get_list(page, sort)` | Fetch video list with caching |
| `search_list(keyword, page, sort)` | Search videos with caching |
| `change_viewed(ban_go)` | Toggle viewed status |
| `save_memory()` | Persist viewed state to `memory.txt` |
| `export_viewed()` | Export viewed list as string |
| `import_viewed(content, mode)` | Import viewed list (append/overwrite) |
| `get_mgs_list()` | Get MGStage prefix list |
| `save_mgs_list(mgs_list)` | Save MGStage prefix list |
| `clear_cache()` | Clear list and search caches |
| `open_url(url)` | Open URL in system browser |

### State Management

`AppState` (in `state.rs`) contains:
- `memory: Mutex<HashSet<String>>` - Viewed video IDs
- `mgs_list: Mutex<HashSet<String>>` - MGStage prefix list
- `config: AppConfig` - Backend IP/port configuration
- `cache: Mutex<HashMap<String, Vec<VideoInfoItemVo>>>` - List cache
- `search_cache: Mutex<HashMap<String, Vec<VideoInfoItemVo>>>` - Search cache

All state is loaded from files at startup and persisted on demand.

### Video ID Types (utils.rs)

The `Type` enum classifies IDs:
- `FC2` - FC2-PPV prefixed IDs
- `Prestige` - IDs with prefixes in `MGSList.txt`
- `Normal` - All other IDs

Preview links are generated based on type:
- Official: DMM (Normal), MGStage (Prestige), FC2 (FC2)
- Third-party: JavStore, JavArchive

### Frontend-to-Rust Communication

Frontend uses `@tauri-apps/api/core` `invoke()`:

```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke('get_list', { page: "1", sort: "uploading" });
```

See `src/components/CommonFunctions.ts` for all API wrappers.

## Configuration Files

Files are stored in the same directory as the executable:

| File | Format | Purpose |
|------|--------|---------|
| `application.json` | JSON | `{ "backend_ip": "...", "backend_port": "5000" }` |
| `memory.txt` | Semicolon-separated | Viewed video IDs |
| `MGSList.txt` | Line-separated | MGStage prefixes |

## Key Dependencies

### Rust (Cargo.toml)
- `tauri` 2.10.0 with plugins: http, opener, fs, dialog, clipboard-manager
- `reqwest` 0.11 - HTTP client
- `serde` - Serialization
- `anyhow` - Error handling

### Frontend (package.json)
- Vue 3.2.13 with Vue Router 4
- Element Plus 2.2.28 - UI component library
- `@tauri-apps/api` 2.10.1 - Tauri bridge

## Caching Strategy

The Rust backend maintains two in-memory caches:
- `cache` - For list requests, keyed by `page + sort`
- `search_cache` - For search requests, keyed by `page + sort + keyword`

Cache is cleared via the `clear_cache` command (no TTL, manual only).

## Legacy Code

- `GoNyaa-Server/` - Java Spring Boot backend, replaced by Rust
- `GoNyaa-Server/web/` - Original Vue frontend, copied to `GoNyaa-Client/src/`
