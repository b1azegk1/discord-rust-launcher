# Discord Launcher (Rust + Tauri 2)

Windows 10/11 desktop launcher for Discord Stable, PTB and Canary.

## Features
- Detects the three Discord channels in the current Windows user's Local AppData.
- Launches the selected channel directly.
- Optional custom launch arguments.
- Requests update checks using each channel's bundled `Update.exe`.
- Discord-inspired dark UI with animated background, channel cards and status messages.

## Build on GitHub Actions
Every push to `main` or `master` starts a Windows x64 build. Open the **Actions** tab and download the `DiscordLauncher-Windows-x64` artifact after the workflow completes.

The artifact includes the app executable and NSIS `.exe` / MSI installers. No Rust, Node.js or Visual Studio Build Tools are needed on your PC.

## Local development
Requirements: Windows 10/11, Node.js 20+, Rust stable, Microsoft C++ Build Tools and WebView2 Runtime.

```powershell
npm install
npm run tauri dev
npm run tauri build
```

Build output: `src-tauri/target/release/bundle/`.

## Notes
- Update checks call Discord's own `Update.exe --checkForUpdates`; the launcher reports that the request was started, not the final update result.
- The launcher does not download Discord from unofficial sources or modify Discord files.
- Detection checks `%LOCALAPPDATA%\Discord`, `DiscordPTB` and `DiscordCanary`, including `app-*` version folders.
- This app is not affiliated with Discord, Inc. and is not digitally signed. Windows SmartScreen may show a warning for unsigned builds.
