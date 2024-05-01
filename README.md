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
7. Example of tauri application getting micrphone access, with recorder which stops recording after 3 seconds of silence. Check `entitlements.plist`, `tauri.conf.json` and `AudioRecorder.vue`
8. Example using [crabnebula](https://github.com/crabnebula-dev/devtools) devtools (https://devtools.crabnebula.dev/dash/127.0.0.1/3000/console)
9. Using sidecar to execute some python executable built with `PyInstaller`, which is run by the JS frontend. (also included dev-tools to inspect the time waterfall of the operations)
10. Using the `tauri-cli` and `cargo tauri icon` command to change the desktop app icon.
11. Creating a custom window menu with `main.rs`
12. Saving a file from an action in frontend frontend to the machine filesystem.
13. Using webpack to load in a Javascript plugin dynamically using the UI. (Need to fix, doesn't work in browser only window)
14. < broken version of example13 >
<!-- 7. Reducing build size: the following example showcases how to optimize builds for size
8. Embedding External Binaries
9. Embedding Additional Files -->
 
# Tauri version >= v2.0

Tauri has a new version coming out with additional capabilities, so separated the examples related to the latest release here: 

15. Setting up a tauri 2 project for the first time with `npm create tauri-app@latest -- --beta`. Using `WebviewUrl` to load in 3 different webviews within the one app. Also have a button on the frontend which can open up another `.app` file.
16. My attempt of extending example15 with an "embedded" app window - tried overlaying the window of the binary when run on top desktop application host window and track the side of the host window so i can be programtically resized. Didn't work as planned.
17. Embedding a WASM as a webview within a host tauri vue application - also included rust WASM backend function to be called from JS frontend. NOTE: html is rended using `web_sys`.
18.
19. Using the tauri store plugin to save and retrieve a `.bin` file key from the JS frontend. Files are saved to `$APPDATA/`  - Can be represented as `BaseDirectory.AppData`.
20. Example of creating an email/password account within supabase Auth
21. Example of logging in with an email/password account OR with Google account within supabase Auth