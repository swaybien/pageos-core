<div align="right" >
  <details>
    <summary >🌐 语言</summary>
    <div>
      <div align="right">
        <p><a href="#">English</a></p>
        <p><a href="README.md">简体中文</a></p>
      </div>
    </div>
  </details>
</div>

# pageos-core

**Web-Centric OS Framework** · [![MPL-2.0 License](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

> Rust backend with WebSocket bridge + HTML5 UI framework for building  
> browser-first Linux distributions. Provides secure system API access  
> and GPU-accelerated Wayland app streaming.

## Core Architecture

- **Display Layer**: Cage + Firefox kiosk mode for single-page app rendering
- **Service Layer**: Rust WebSocket server
- **App Ecosystem**: Local/online web application repositories
- **System Integration**: System calls and hardware access via WebSocket

## Key Features

- 🖥️ Web application repository management (local/online)
- 🔒 Fine-grained permission control system
- 🔌 System call APIs (reboot, command execution, etc.)
- 🚀 GPU-accelerated Wayland application streaming
- 📦 Unified application package format (metadata.json)

## Repository Components

- [pageos-core](https://github.com/swaybien/pageos-core): Core service (current repo)
- [pageos-greet](https://github.com/swaybien/pageos-greet): Login manager
- [pageos-apps](https://github.com/swaybien/pageos-apps): Official web app repository
- [pageos-pkgr](https://github.com/swaybien/pageos-pkgr): Repository manager
- [pageos-pkgr-ui](https://github.com/swaybien/pageos-pkgr-ui): Repository manager GUI

## Quick Start

```bash
# Start core service
pageos-core -p 12800 --command "cage -s -- firefox --kiosk --no-remote http://127.0.0.1:12800"
```

## Documentation

See [docs/](docs/) directory
