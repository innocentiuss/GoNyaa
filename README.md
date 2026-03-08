# GoNyaa

GoNyaa is now a two-part project:

- `GoNyaa-Crawler`: the remote crawler service that scrapes `sukebei.nyaa.si`
- `GoNyaa-Desktop`: the local desktop app built with Tauri, Rust, and Vue

The old local Spring Boot server has been replaced by the desktop app runtime. The crawler contract remains compatible with `/app` and `/search`.

## Project Layout

```text
GoNyaa/
  GoNyaa-Crawler/
  GoNyaa-Desktop/
```

## Components

### GoNyaa-Crawler

Responsibilities:

- Scrape `sukebei.nyaa.si`
- Normalize crawler results
- Expose `/app` and `/search`

Local run:

```bash
cd GoNyaa-Crawler
pip install flask requests beautifulsoup4 lxml
python Nyaa_Get.py
```

### GoNyaa-Desktop

Responsibilities:

- Call the remote crawler from Rust
- Maintain `memory.txt` and `MGSList.txt`
- Merge viewed state locally
- Handle MGStage prefixes, import/export, and external link opening
- Host the Vue UI inside Tauri

Local run:

```bash
cd GoNyaa-Desktop
npm install
npm run tauri:dev
```

Frontend-only build:

```bash
cd GoNyaa-Desktop
npm run build
```

Rust compile check:

```bash
cd GoNyaa-Desktop/src-tauri
cargo check
```

## Data Compatibility

The desktop app keeps compatibility with:

- `memory.txt`
- `MGSList.txt`

On first launch it migrates legacy files into the Tauri app data directory if they already exist in an old `GoNyaa-Server` working directory.

## Configuration

The desktop app reads the crawler base URL from:

- `GONYAA_CRAWLER_BASE_URL`

If not set, it defaults to the current production crawler endpoint already wired into the app.

## Packaging

Build the desktop app bundle with:

```bash
cd GoNyaa-Desktop
npm run tauri:build
```

This requires the normal Tauri Windows prerequisites on the machine.