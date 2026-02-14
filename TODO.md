# GoNyaa Refactoring Guide (Java -> Rust + Tauri)

This guide details the steps to refactor the GoNyaa project from a Java SpringBoot backend to a Rust + Tauri desktop application, maintaining the existing Vue frontend.

## Prerequisites
- Rust (Cargo) installed.
- Node.js (NVM) installed.
- `GoNyaa-Server` directory contains the source code.

## Phase 1: Project Initialization

- [x] **Initialize Tauri Project**
    1.  Open terminal in `GoNyaa-Server/web`.
    2.  Run `npm install @tauri-apps/cli`.
    3.  Run `npx tauri init`.
        -   **App Name**: `GoNyaa`
        -   **Window Title**: `GoNyaa`
        -   **Web Assets Path**: `../dist` (Note: Vue build output is usually `dist` in the project root, verify `vue.config.js` if unsure. Default is `dist` relative to `package.json` location?)
        -   Wait, `npx tauri init` asks for relative path. If running in `web`, `dist` is usually `./dist`.
        -   **Dev URL**: `http://localhost:8080` (Standard Vue dev server).
        -   **Frontend Dev Command**: `npm run serve`.
        -   **Frontend Build Command**: `npm run build`.
    4.  Verify `src-tauri` directory is created inside `web` (or root if you ran it there, but best inside `web` or parallel? Standard is specific `src-tauri` folder). Standard Tauri practice is `src-tauri` *next* to `package.json` if possible, or `src-tauri` at root and frontend in `src`.
    -   *Correction*: GoNyaa has `GoNyaa-Server/web`. So `src-tauri` should probably be `GoNyaa-Server/web/src-tauri` (if `web` is the root of the GUI app) OR `GoNyaa-Client/src-tauri`.
    -   **Re-evaluating Structure**: The user wants `GoNyaa-Client`.
    -   **Recommendation**: Create a NEW directory `GoNyaa-Client` sibling to `GoNyaa-Server`. Copy `GoNyaa-Server/web` content into `GoNyaa-Client`. Then init Tauri there. This is cleaner and leaves `GoNyaa-Server` (Java) intact as reference/backup.
    -   **Action**: Create `GoNyaa-Client` directory. Copy `web` contents. Init Tauri.

- [x] **Configure Tauri** (`src-tauri/tauri.conf.json`)
    - [x] Update `identifier` to `com.bubble.gonyaa`.
    - [x] Enable allowlist for required APIs:
        -   `http`: `all: true` (for calling Python backend).
        -   `fs`: `all: true` (for `memory.txt`).
        -   `path`: `all: true`.
        -   `shell`: `open: true` (for `window.open` handling or explicit shell open).
        -   `dialog`: `all: true` (for import/export).
    - [x] Configure `security`:
        -   `csp`: Ensure `connect-src` allows the Python backend IP.

## Phase 2: Backend Implementation (Rust)

- [x] **Add Dependencies** (`src-tauri/Cargo.toml`)
    ```toml
    [dependencies]
    tauri = { version = "1", features = ["http-all", "shell-open", "fs-all", "path-all", "dialog-all", "clipboard-all"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    reqwest = { version = "0.11", features = ["json", "blocking"] } 
    lazy_static = "1.4"
    anyhow = "1.0"
    base64 = "0.21"
    encoding_rs = "0.8" # Optional, if needed
    ```

- [x] **Define Data Models** (`src-tauri/src/models.rs`)
    - [x] `VideoInfoItemVo`:
        ```rust
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct VideoInfoItemVo {
            pub date: String,
            pub fanHao: String,
            pub downCnt: String,
            pub upCnt: String,
            pub magnetLink: String,
            pub size: String,
            pub title: String,
            pub finCnt: String,
            pub type_: String, // "type" is reserved keyword
            pub viewLink: String,
            pub viewLink2: String,
            pub viewLink3: String,
            pub viewed: bool,
            pub isViewed: String, // "√" or "×"
        }
        ```
    - [x] `WebResponse<T>` wrapper.

- [x] **Implement State Management** (`src-tauri/src/state.rs`)
    - [x] `AppState`:
        ```rust
        pub struct AppState {
            pub memory: Mutex<HashSet<String>>,
            pub mgs_list: Mutex<HashSet<String>>,
            pub config: AppConfig,
            pub cache: Mutex<HashMap<String, Vec<VideoInfoItemVo>>>
        }
        ```

- [x] **Implement File Operations** (`src-tauri/src/file_ops.rs`)
    - [x] `load/save_memory()`: Read/Write `memory.txt` (semicolon separated).
    - [x] `load/save_mgs_list()`: Read/Write `MGSList.txt` (newline separated).
    - [x] `load_config()`: Read `application.json`.

- [x] **Implement Commands** (`src-tauri/src/main.rs` or `commands.rs`)
    - [x] `get_list(page, sort, state)`
    - [x] `search_list(keyword, page, sort, state)`
    - [x] `change_viewed(ban_go, state)`
    - [x] `save_memory(state)`
    - [x] `get_mgs_list(state)`
    - [x] `save_mgs_list(list, state)`
    - [x] `clear_cache(state)`
    - [x] `import_viewed(file, mode, state)`
    - [x] `export_viewed(state)` -> Return content or write file.

## Phase 3: Frontend Refactoring

- [x] **Migrate `CommonFunctions.ts`**
    - [x] Replace `axios` with `invoke` from `@tauri-apps/api`.
    - [x] Map functions:
        -   `getData` -> `invoke('get_list', { ... })`
        -   `searchData` -> `invoke('search_list', { ... })`
        -   `changeViewed` -> `invoke('change_viewed', { ... })`
        -   Etc.

- [x] **Update `package.json`**
    -   Ensure `scripts` has `tauri` commands.

## Phase 4: Verification

- [ ] **Build & Run**
    -   Run `npm run tauri dev` to test locally.
    -   Verify data loading from Python backend.
    -   Verify `memory.txt` reading/writing.
    -   Verify `MGSList.txt` reading/writing.

## Configuration File Template (application.json)
```json
{
  "backend_ip": "43.130.243.211",
  "backend_port": 5000
}
```
