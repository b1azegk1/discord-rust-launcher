#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy, Debug)]
enum Channel {
    Stable,
    Ptb,
    Canary,
}

impl Channel {
    fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "stable" => Ok(Self::Stable),
            "ptb" => Ok(Self::Ptb),
            "canary" => Ok(Self::Canary),
            _ => Err(format!("Неизвестный канал Discord: {value}")),
        }
    }

    fn folder(self) -> &'static str {
        match self {
            Self::Stable => "Discord",
            Self::Ptb => "DiscordPTB",
            Self::Canary => "DiscordCanary",
        }
    }

    fn executable(self) -> &'static str {
        match self {
            Self::Stable => "Discord.exe",
            Self::Ptb => "DiscordPTB.exe",
            Self::Canary => "DiscordCanary.exe",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Stable => "Discord Stable",
            Self::Ptb => "Discord PTB",
            Self::Canary => "Discord Canary",
        }
    }
}

#[derive(Clone, Serialize)]
struct InstallInfo {
    installed: bool,
    path: Option<String>,
    version: Option<String>,
    updater_path: Option<String>,
}

fn local_app_data() -> Result<PathBuf, String> {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or_else(|| "Windows не вернула переменную LOCALAPPDATA".to_string())
}

fn version_from_path(path: &Path) -> Option<String> {
    path.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .filter(|name| name.starts_with("app-"))
        .map(|name| name.trim_start_matches("app-").to_string())
}

fn find_executable(root: &Path, exe_name: &str) -> Option<(PathBuf, Option<String>)> {
    let direct = root.join(exe_name);
    if direct.is_file() {
        return Some((direct, None));
    }

    let mut candidates: Vec<PathBuf> = fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("app-"))
                    .unwrap_or(false)
        })
        .collect();

    candidates.sort();
    candidates.reverse();

    for folder in candidates {
        let candidate = folder.join(exe_name);
        if candidate.is_file() {
            let version = version_from_path(&candidate);
            return Some((candidate, version));
        }
    }
    None
}

fn detect_one(channel: Channel) -> Result<InstallInfo, String> {
    let root = local_app_data()?.join(channel.folder());
    let updater = root.join("Update.exe");
    let found = if root.is_dir() {
        find_executable(&root, channel.executable())
    } else {
        None
    };

    Ok(match found {
        Some((path, version)) => InstallInfo {
            installed: true,
            path: Some(path.to_string_lossy().into_owned()),
            version,
            updater_path: if updater.is_file() {
                Some(updater.to_string_lossy().into_owned())
            } else {
                None
            },
        },
        None => InstallInfo {
            installed: false,
            path: None,
            version: None,
            updater_path: None,
        },
    })
}

#[derive(Serialize)]
struct AllInstallations {
    stable: InstallInfo,
    ptb: InstallInfo,
    canary: InstallInfo,
}

#[tauri::command]
fn detect_discord() -> Result<AllInstallations, String> {
    Ok(AllInstallations {
        stable: detect_one(Channel::Stable)?,
        ptb: detect_one(Channel::Ptb)?,
        canary: detect_one(Channel::Canary)?,
    })
}

fn parse_args(input: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && quote.is_some() {
            escaped = true;
            continue;
        }
        if ch == '"' || ch == '\'' {
            match quote {
                Some(q) if q == ch => quote = None,
                None => quote = Some(ch),
                _ => current.push(ch),
            }
            continue;
        }
        if ch.is_whitespace() && quote.is_none() {
            if !current.is_empty() {
                args.push(std::mem::take(&mut current));
            }
        } else {
            current.push(ch);
        }
    }

    if quote.is_some() {
        return Err("В аргументах запуска не закрыта кавычка".to_string());
    }
    if !current.is_empty() {
        args.push(current);
    }
    Ok(args)
}

#[tauri::command]
fn launch_discord(channel: String, args: String) -> Result<String, String> {
    let channel = Channel::parse(&channel)?;
    let install = detect_one(channel)?;
    let executable = install.path.ok_or_else(|| {
        format!("{} не найдена. Установи её с официального сайта Discord.", channel.label())
    })?;

    let parsed_args = parse_args(&args)?;
    let mut command = Command::new(&executable);
    command.args(parsed_args);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }

    command.spawn().map_err(|e| format!("Не удалось запустить {}: {e}", channel.label()))?;
    Ok(format!("{} запускается", channel.label()))
}

#[tauri::command]
fn check_updates(channels: Vec<String>) -> Result<Vec<String>, String> {
    let mut messages = Vec::new();

    for value in channels {
        let channel = Channel::parse(&value)?;
        let install = detect_one(channel)?;
        let updater = match install.updater_path {
            Some(path) => PathBuf::from(path),
            None => {
                messages.push(format!("{}: Update.exe не найден", channel.label()));
                continue;
            }
        };

        let mut command = Command::new(updater);
        command.arg("--checkForUpdates");

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }

        match command.spawn() {
            Ok(_) => messages.push(format!("{}: запрос проверки отправлен", channel.label())),
            Err(error) => messages.push(format!("{}: ошибка запуска проверки ({error})", channel.label())),
        }
    }

    Ok(messages)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            detect_discord,
            launch_discord,
            check_updates
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска Discord Launcher");
}
