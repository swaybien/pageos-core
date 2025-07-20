<div align="right" >
  <details>
    <summary >🌐 语言</summary>
    <div>
      <div align="right">
        <p><a href="README.en.md">English</a></p>
        <p><a href="#">简体中文</a></p>
      </div>
    </div>
  </details>
</div>

# pageos-core

**Web-Centric OS 框架** · [![MPL-2.0 License](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

> 基于 Rust 的后端服务 + HTML5 前端框架，用于构建以浏览器为核心的 Linux 发行版。
> 提供安全的系统 API 访问和 GPU 加速的 Wayland 应用流式传输。

## 核心架构

- **显示层**：Cage + Firefox kiosk 模式渲染单页应用
- **服务层**：Rust 实现的 WebSocket 服务端
- **应用生态**：本地/在线网页应用仓库系统
- **系统集成**：通过 WebSocket 实现系统调用和硬件访问

## 主要功能

- 🖥️ 网页应用仓库管理（本地/在线）
- 🔒 细粒度权限控制系统
- 🔌 系统调用 API（重启、命令执行等）
- 🚀 GPU 加速的 Wayland 应用流式传输
- 📦 统一的应用包格式（metadata.json）

## 仓库组件

- [pageos-core](https://github.com/swaybien/pageos-core)：核心服务（当前仓库）
- [pageos-greet](https://github.com/swaybien/pageos-greet)：登录管理器
- [pageos-apps](https://github.com/swaybien/pageos-apps)：官方网页应用仓库
- [pageos-pkgr](https://github.com/swaybien/pageos-pkgr)：仓库管理工具
- [pageos-pkgr-ui](https://github.com/swaybien/pageos-pkgr-ui)：仓库管理 GUI

## 快速开始

```bash
# 启动核心服务
pageos-core -p 12800 --command "cage -s -- firefox --kiosk --no-remote http://127.0.0.1:12800"
```

## 开发文档

详见 [docs/](docs/) 目录
