# XIVLauncher NEO
Concept for a XIVLauncher rewrite in Rust, UI via Vue + Tauri.

## Concepts
Due to this being a split project/library of sorts, a delineation needs to be made between what should be accomplished in the UI (src) and in Rust (src-tauri).
As a general rule, control of the business logic begins in the UI. That is to say, if something needs to be run, it should typically be from a user interaction. 
- Updates and notifications should be wired through Tauri events, with the initial status returned via a `Result`.
- Errors should be returned as a serializable `Error` so that they can be shown in an error dialog.

While the configuration objects are available as statics in Rust, they should not be modified, otherwise the UI will fall out of sync with what is stored in the library. Perform whatever  operation needs doing, return the result, and let the UI update the settings.

## Environment setup
- Install `nvm` for [Windows](https://github.com/coreybutler/nvm-windows) 
- Install `rustup` for [Windows](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

```pwsh
nvm install lts

# nvm use lts (Requires admin)
Start-Process cmd "/k nvm install lts" -Verb RunAs

npm install --global yarn
```

## Project setup
```
yarn --cwd gui install
```

### Compiles and hot-reloads for development
```
yarn --cwd gui tauri dev
```

### Compiles and minifies for production
```
yarn --cwd gui tauri build
```

### Lints and fixes files
```
yarn --cwd gui lint
```
