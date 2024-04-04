# tauri-projects
Central repository for all learnings with Tauri &amp; Rust Backend

# Tauri version < v1.6.1

```bash
npm create tauri-app@latest
npm install --save-dev @tauri-apps/cli
npm run tauri dev
```

1. A simple boilerplate example with vanilla JS and html to call a function in Tauri using `withGlobalTauri` 
2. A simple boilerplate example with vite to call a function in Tauri using `@tauri-apps/api`, and printing from the rust backend
3. Using control the inspector window visibility by using the `Window::open_devtools` and `Window::close_devtools` functions
4. Example of how to use the Isolation pattern to ensure app security, but isolating the app in a sandbox. (`window.__TAURI_ISOLATION_HOOK__`)
5. Example using `vscode-lldb` vs code extension to enable debugging in any rust file, must add `.vscode/launch.json` and `.vscode/tasks.json`


6. An example using `tauri-apps/tauri-action@v0` github action to automate building for MacOS, Windows and Linux operating systems. Check `.github/workflows/build.yaml`
<!-- 7. Reducing build size: the following example showcases how to optimize builds for size
8. Embedding External Binaries
9. Embedding Additional Files -->



# Tauri version >= v2.0

Tauri has a new version coming out with additional capabilities, so separated the examples related to the latest release here: 
