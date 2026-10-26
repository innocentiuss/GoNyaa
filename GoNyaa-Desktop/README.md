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

## Browser and Preview Link Settings

Open **设置** on the home or search page to configure:

- The system default browser, Chrome, Edge, or Firefox. For a named browser, the app searches common installation locations and PATH; you can also select its executable manually (including a portable installation).
- Private browsing for Chrome, Edge, or Firefox. Select a specific browser to enable this option. It applies to individual preview links and batch opening.
- The display names and URL templates for the two third-party preview links. Include `{id}` where the video ID belongs, for example `https://example.com/search?q={id}`. IDs are URL encoded; query strings and fragments are supported. **恢复默认链接** restores JavStore and JavArchive.

These settings are saved alongside the crawler address in `settings.json` in the app data directory and survive restarts. Existing settings files receive the new defaults automatically. Updating link templates also updates links from cached lists. Magnet links continue to use the system's registered handler.

On Windows, left-click a preview link to open it normally, or middle-click to open it while keeping GoNyaa in front for continued browsing. Middle-click temporarily protects GoNyaa from browser focus changes for up to five seconds. Clicking another window or using Alt to switch windows releases that protection immediately. This keeps the desktop app in front; it does not control the browser's selected tab.

## Current Scope

This desktop app replaces the old local Spring Boot server.
The crawler remains a separate service and is intentionally not migrated in this phase.
