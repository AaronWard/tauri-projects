


### 1. Install updated plugin

```bash
npm install @tauri-apps/plugin-updater  @tauri-apps/plugin-dialog
```

### 2. Generate key

```bash
npm run tauri signer generate -- -w ~/.tauri/myapp.key
```

for windows:
```
npm run tauri signer generate -- -w $HOME/.tauri/myapp.key

```


```bash
(base) example26 award40$ npm run tauri signer generate -- -w ~/.tauri/myapp.key

> example26@0.0.0 tauri
> tauri signer generate -w /Users/award40/.tauri/myapp.key

Please enter a password to protect the secret key.
Password: 

```

### 3. Create a complex password 

```bash
Deriving a key from the password in order to encrypt the secret key... done

Your keypair was generated successfully
Private: /Users/award40/.tauri/myapp.key (Keep it secret!)
Public: /Users/award40/.tauri/myapp.key.pub
---------------------------

Environment variables used to sign:
`TAURI_SIGNING_PRIVATE_KEY`  Path or String of your private key
`TAURI_SIGNING_PRIVATE_KEY_PASSWORD`  Your private key password (optional)

ATTENTION: If you lose your private key OR password, you'll not be able to sign your update package and updates will not work.
---------------------------

(base) LAMU0CLP74YXVX6:example26 award40$ 

```

save these, and never lose them!


### 4. Add generate keys as variables

```bash
export TAURI_SIGNING_PRIVATE_KEY="/Users/award40/.tauri/myapp.key"
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="/Users/award40/.tauri/myapp.key.pub"
```

windows:
```powershell
$env:TAURI_SIGNING_PRIVATE_KEY="$HOME/.tauri/myapp.key"
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$HOME/.tauri/myapp.key.pub"
```



### 5. Update `tauri.conf.json`


Change default identifier value:

```
"identifier": "com.tauri.dev"
```

Update values:

```json
{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      "pubkey": "CONTENT FROM PUBLICKEY.PEM",
      "endpoints": [
        "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}",
        // or a static github json file
        "https://github.com/user/repo/releases/latest/download/latest.json"
      ]
    }
  }
}
```



Full `tauri.conf.json`:

```json
{
  "productName": "example26",
  "version": "0.0.0",
  "identifier": "com.updater.dev",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "example26",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "createUpdaterArtifacts": true,
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  },
  "plugins": {
    "updater": {
      "windows": {
        "installMode": "passive"
      },
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDY1ODNCRDU1NDQzQ0Y5RDQKUldUVStUeEVWYjJEWmJNVkNKamgyTUw0L29tUlc2NGpaUjh6aEdabjRZWGZlZHYvc3FZVlo4bjgK",
      "endpoints": [
        "https://github.com/AaronWard/tauri-projects/releases/latest/download/latest.json"
      ]
    }
  }
}
```

NEED TO CONFIRM: can `endpoints` be set to private repositories?


### 6. Add Update Check Button in UI

/src/components/greet.vue:

```js
// sample front-end code for the updater
import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';

async function checkForAppUpdates() {
  const update = await check();
  if (update === null) {
			await message('Failed to check for updates.\nPlease try again later.', { 
				title: 'Error',
				kind: 'error',
				okLabel: 'OK'
			});
			return;
		} else if (update?.available) {
    const yes = await ask(`Update to ${update.version} is available!\n\nRelease notes: ${update.body}`, { 
      title: 'Update Available',
      kind: 'info',
      okLabel: 'Update',
      cancelLabel: 'Cancel'
    });
    if (yes) {
      await update.downloadAndInstall();
      // Restart the app after the update is installed by calling the Tauri command that handles restart for your app
      // It is good practice to shut down any background processes gracefully before restarting
      // As an alternative, you could ask the user to restart the app manually
      await invoke("graceful_restart");
    }
  } else if (onUserClick) {
    await message('You are on the latest version. Stay awesome!', { 
      title: 'No Update Available',
      kind: 'info',
      okLabel: 'OK'
    });
  }
}
```

### 7. Add the relevant permissions in your `capabilities > main.json` file.


```json
{
  "permissions": [
    ...
    "dialog:default",
    "dialog:allow-ask",
    "dialog:allow-message",
    "updater:default",
    "updater:allow-check",
    "updater:allow-download-and-install"
  ]
}
```

## Make latest.json

```json
{
  "version": "v0.0.1",
  "notes": "Your Release Notes go here",
  "pub_date": "2020-06-22T19:25:57Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "Content of app.tar.gz.sig",
      "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-x86_64.app.tar.gz"
    },
    "darwin-aarch64": {
      "signature": "Content of app.tar.gz.sig",
      "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-aarch64.app.tar.gz"
    },
    "linux-x86_64": {
      "signature": "Content of app.AppImage.tar.gz.sig",
      "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-amd64.AppImage.tar.gz"
    },
    "windows-x86_64": {
      "signature": "Content of app-setup.nsis.sig or app.msi.sig, depending on the chosen format",
      "url": "https://github.com/username/reponame/releases/download/v1.0.0/app-x64-setup.nsis.zip"
    }
  }
}
```

- version can be with or without the leading v.
notes can be any string.
- pub_date should be in the format YYYY-MM-DDTHH:MM:SSZ.
- platforms can contain the platforms you are targeting. The keys should be the platform names and the values should be objects with signature and url keys.
- signature should be the content of the signature file generated when you build for the respective platform.
- url should be the URL to the binary file from the release. Don’t accidentally put the URL of the release page itself.


> is `universal-apple-darwin` supported?
> https://discord.com/channels/616186924390023171/1213374366981693490/1213374959183859754

---

- Set the same project version in the `Cargo.toml`, `tauri.conf.json` and `package.json` files - `0.0.1`
- https://thatgurjot.com/til/tauri-auto-updater/
- `allow-restart` - https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/process/permissions/autogenerated/reference.md
- https://v2.tauri.app/reference/javascript/process/#relaunch - Relaunch via javascript
- https://docs.rs/tauri/2.0.0-beta.22/tauri/process/fn.restart.html - restart using rust
- https://docs.rs/tauri/latest/tauri/struct.AppHandle.html#method.restart