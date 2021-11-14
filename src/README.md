# XIVLauncher NEO
Concept for a XIVLauncher rewrite in Rust, UI via Vue + Tauri.

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

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
