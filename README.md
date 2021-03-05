# lightsheet_rust


### Compiles and hot-reloads for development
```
yarn tauri:serve
```

### debug
```
yarn serve
```

```
==== OSX ====
cd src-tauri
TAURI_CONFIG='{
    "ctx": {},
    "build":{
      "devPath": "http://localhost:8081"
    },
    "tauri": {
      "embeddedServer": {
        "active": true
      },
      "bundle": {
        "active": true, 
        "targets": "all",
        "identifier": "com.tauri.dev",
        "icon": [
          "icons/32x32.png",
          "icons/128x128.png",
          "icons/128x128@2x.png",
          "icons/icon.icns",
          "icons/icon.ico"
        ],
        "resources": [],
        "externalBin": [],
        "copyright": "",
        "category": "DeveloperTool",
        "shortDescription": "",
        "longDescription": "",
        "deb": {
          "depends": [],
          "useBootstrapper": false
        },
        "osx": {
          "frameworks": [],
          "minimumSystemVersion": "",
          "useBootstrapper": false
        },
        "exceptionDomain": ""
      },
      "allowlist": {
        "all": true
      },
      "window": {
        "title": "lightsheet",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      },
      "security": {
        "csp": "default-src blob: data: filesystem: ws: http: https: 'unsafe-eval' 'unsafe-inline'"
      },
      "inliner": {
        "active": true
      }
    }
  }' cargo build
```

### Compiles and minifies for production
```
yarn tauri:build
```


### Compile rust
```
OUT_DIR="D:/workspace/vscode/lightsheet_rust/src-tauri" cargo build
```