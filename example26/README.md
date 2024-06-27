


### 1. Install updated plugin

```bash
npm install @tauri-apps/plugin-updater
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



### Update `tauri.cong.json`


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
}```



Full `tauri.conf.json`:
```bash
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
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDY1ODNCRDU1NDQzQ0Y5RDQKUldUVStUeEVWYjJEWmJNVkNKamgyTUw0L29tUlc2NGpaUjh6aEdabjRZWGZlZHYvc3FZVlo4bjgK",
      "endpoints": [
        "https://github.com/AaronWard/tauri-projects/releases/latest/download/latest.json"
      ]
    }
  }
}
```