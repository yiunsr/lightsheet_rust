SET TAURI_CONFIG= ^
{ ^
  "$schema": "..\\node_modules/@tauri-apps/cli\\schema.json", ^
  "build":{ ^
    "devPath": "http://localhost:8080", ^
    "distDir": "../dist", ^
    "beforeBuildCommand": "yarn build", ^
    "withGlobalTauri": true}, ^
  "package": { ^
    "productName": "lightsheet_rust", ^
    "version": "0.1.0" ^
  }, ^
  "tauri": { ^
    "allowlist": { ^
      "all": true ^
    }, ^
    "bundle": { ^
      "active": true,  ^
      "targets": "all", ^
      "identifier": "com.labstoo.lightsheet", ^
      "icon": [ ^
        "icons/32x32.png", ^
        "icons/128x128.png", ^
        "icons/128x128@2x.png", ^
        "icons/icon.icns", ^
        "icons/icon.ico" ^
      ], ^
      "resources": [], ^
      "externalBin": [], ^
      "copyright": "", ^
      "category": "DeveloperTool", ^
      "shortDescription": "", ^
      "longDescription": "", ^
      "deb": { ^
        "depends": [] ^
      }, ^
      "macOS": { ^
        "entitlements": null, ^
        "exceptionDomain": "", ^
        "frameworks": [], ^
        "providerShortName": null, ^
        "signingIdentity": null ^
      } ^
    }, ^
    "windows": [{ ^
      "title": "lightsheet", ^
      "width": 800, ^
      "height": 600, ^
      "resizable": true, ^
      "fullscreen": false ^
    }], ^
    "security": { ^
      "csp": "default-src blob: data: filesystem: ws: http: https: 'unsafe-eval' 'unsafe-inline'" ^
    } ^
  } ^
} 
cargo build --no-default-features