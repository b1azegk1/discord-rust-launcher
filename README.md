# Discord Rust Launcher

A Windows 10/11 desktop shell built with Rust + Tauri 2. It opens the official Discord web app inside its own window.

## Build on Windows

Requirements:
- Windows 10 or 11 (x64)
- Node.js 20+
- Rust stable (install with https://rustup.rs/)
- Microsoft C++ Build Tools with the MSVC toolchain
- WebView2 Runtime

After installing the requirements, double-click **build-windows.bat** in the project folder. It installs the frontend packages and runs the Tauri release build.

The installers are written to:

`src-tauri/target/release/bundle/`

The standalone executable is:

`src-tauri/target/release/discord-launcher.exe`

You can also build from PowerShell:

```powershell
npm install
npm run tauri build
```

## GitHub Actions

Every push to `main` or `master` starts a Windows x64 build. Open the **Actions** tab and download the `DiscordLauncher-Windows-x64` artifact after the workflow completes.

Rust compilation is cached between builds, and superseded builds on the same branch are cancelled automatically.

## Notes

- The embedded client loads https://discord.com/app. Sign in with your Discord account there.
- This project does not bundle or modify the official Discord desktop application.
- PTB and Canary are separate desktop release channels; this embedded web client uses the official Discord web app.
- This app is not affiliated with Discord, Inc. and is not digitally signed. Windows SmartScreen may show a warning for unsigned builds.
