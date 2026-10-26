use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::domain::settings::{BrowserKind, BrowserSettings};
use crate::infra::errors::{AppError, AppResult};

pub fn resolve_executable(settings: &BrowserSettings) -> AppResult<PathBuf> {
    if !settings.executable_path.is_empty() {
        let path = PathBuf::from(&settings.executable_path);
        if !path.is_absolute() || !path.is_file() {
            return Err(AppError::Config(
                "请选择存在的浏览器可执行文件（绝对路径）".into(),
            ));
        }
        #[cfg(target_os = "windows")]
        if !path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
        {
            return Err(AppError::Config("浏览器路径必须指向 .exe 文件".into()));
        }
        return Ok(path);
    }

    let names: &[&str] = match settings.kind {
        BrowserKind::Chrome => &[
            "chrome.exe",
            "google-chrome",
            "chromium",
            "chromium-browser",
        ],
        BrowserKind::Edge => &["msedge.exe", "microsoft-edge"],
        BrowserKind::Firefox => &["firefox.exe", "firefox"],
        BrowserKind::Default => return Err(AppError::Config("请先选择浏览器".into())),
    };
    let mut candidates = Vec::new();
    #[cfg(target_os = "windows")]
    {
        let relative = match settings.kind {
            BrowserKind::Chrome => "Google/Chrome/Application/chrome.exe",
            BrowserKind::Edge => "Microsoft/Edge/Application/msedge.exe",
            BrowserKind::Firefox => "Mozilla Firefox/firefox.exe",
            BrowserKind::Default => unreachable!(),
        };
        for root in ["LOCALAPPDATA", "PROGRAMFILES", "PROGRAMFILES(X86)"] {
            if let Some(path) = std::env::var_os(root) {
                candidates.push(PathBuf::from(path).join(relative));
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        let relative = match settings.kind {
            BrowserKind::Chrome => "Google Chrome.app/Contents/MacOS/Google Chrome",
            BrowserKind::Edge => "Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            BrowserKind::Firefox => "Firefox.app/Contents/MacOS/firefox",
            BrowserKind::Default => unreachable!(),
        };
        candidates.push(PathBuf::from("/Applications").join(relative));
        if let Some(home_dir) = std::env::var_os("HOME") {
            candidates.push(PathBuf::from(home_dir).join("Applications").join(relative));
        }
    }
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path).filter(|dir| dir.is_absolute()) {
            candidates.extend(names.iter().map(|name| dir.join(name)));
        }
    }
    candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| AppError::Config("未找到所选浏览器，请在设置中指定可执行文件路径".into()))
}

fn browser_args(settings: &BrowserSettings, url: &str) -> Vec<String> {
    let mut args = Vec::new();
    if settings.private_mode {
        args.push(
            match settings.kind {
                BrowserKind::Chrome => "--incognito",
                BrowserKind::Edge => "--inprivate",
                BrowserKind::Firefox => "--private-window",
                BrowserKind::Default => unreachable!("default browser cannot use private mode"),
            }
            .to_string(),
        );
    }
    args.push(url.to_string());
    args
}

pub fn open_url(settings: &BrowserSettings, url: &str) -> AppResult<()> {
    let parsed = reqwest::Url::parse(url).map_err(|error| AppError::Open(error.to_string()))?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err(AppError::Open(
            "预览链接必须是 http:// 或 https:// 网址".into(),
        ));
    }
    if settings.kind == BrowserKind::Default {
        if settings.private_mode {
            return Err(AppError::Config("使用无痕模式时，请指定浏览器".into()));
        }
        return open::that_detached(url).map_err(|error| AppError::Open(error.to_string()));
    }
    let executable = resolve_executable(settings)?;
    let mut command = Command::new(executable);
    command
        .args(browser_args(settings, url))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command
        .spawn()
        .map_err(|error| AppError::Open(error.to_string()))?;
    // Reap the launcher without blocking the Tauri command or the UI.
    std::thread::spawn(move || {
        let _ = child.wait();
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_flags_and_url_are_separate_arguments() {
        let url = "https://example.com/?q=A&B=hello%20world#test";
        for (kind, flag) in [
            (BrowserKind::Chrome, "--incognito"),
            (BrowserKind::Edge, "--inprivate"),
            (BrowserKind::Firefox, "--private-window"),
        ] {
            let mut settings = BrowserSettings {
                kind,
                private_mode: true,
                executable_path: String::new(),
            };
            assert_eq!(browser_args(&settings, url), vec![flag, url]);
            settings.private_mode = false;
            assert_eq!(browser_args(&settings, url), vec![url]);
        }
    }

    #[test]
    fn rejects_non_web_links_before_launching() {
        for url in [
            "file:///C:/test",
            "javascript:alert(1)",
            "--incognito",
            "magnet:?xt=foo",
        ] {
            assert!(open_url(&BrowserSettings::default(), url).is_err());
        }
    }
}
