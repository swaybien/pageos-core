# 草稿

## 构思

- 【历史上下文】：（关于 PJ568 的身份、其团队“SwayBien/随变”和项目管理和命名的信息，此处省略）
- PJ568：

  > 我打算做项目：基于 Arch Linux 的图形化发行版。
  >
  > 该发行版采用 Wayland 下的 cage 显示全屏的（Firefox）浏览器，浏览器用 [kiosk](https://support.mozilla.org/zh-CN/kb/firefox-enterprise-kiosk-mode) 模式打开单个 HTML 页面作为用户界面。任务栏、窗口管理等都在这个页面里实现。用户的所有交互也都在这个页面里实现。
  >
  > 还打算做 Rust 双向服务端程序使用 WebSocket 双向通信使这个 HTML 页面能和系统交互。比如 localhost:<服务端端口>/reboot 接口能重启系统等。
  >
  > 我思考一些问题：
  >
  > 1. 怎么在这样的页面中和 Wayland 下的图形 Linux 程序交互？
  >    终端模拟器的实现可以使用 ttyd 创建网页命令行页面，在 HTML 用户界面里打开内容是指向 ttyd 页面的 iframe 的窗口，但其它传统 Wayland 软件怎么办？
  >    用 cage 启动应用加 wayvnc 再套 noVNC 吗？还是在双向服务端实现 Wayland2HTML 合成器接口之类的？最好是支持服 GPU 渲染的方案，这样玩游戏，播视频就不卡。
  > 2. 浏览器的实现：我如何在操作页面上访问非同源的网页？似乎不是所有的网页支持在 iframe 中跨域显示。
  > 3. 打算给这个操作系统打造网页应用生态，应该给这个系统设计一套“网页应用仓库”服务。
  >    每个网页应用程序主要包含两个部分：网页（index.html、CSS、JS 和其它资源等）、应用元信息（应用名称、`用户名/仓库名`组成的包名、版本、权限信息等）（可选择以文件形式存在，如 `metadata.json`，或内嵌在 `index.html` 中）。
  >    网页应用仓库分为本地和在线，当用户访问一个本地应用（如“设置管理器”）时，实际上是在用内嵌浏览器访问本地仓库中的网页（如 `http://127.0.0.1:<服务端端口>/software/pageos/settings-manager/index.html`）；当用户访问一个在线应用时，实际是访问官方（或镜像）在线软件源里的网页。用户可以选择将在线软件源里的应用下载到本地以实现“安装”。

- DeepSeek-R1-0528：

  > 有创意！针对提出的技术问题的详细分析和建议方案：
  >
  > ### 🧩 技术问题解决方案
  >
  > #### 1. Wayland 应用集成方案
  >
  > - **推荐方案**: WayVNC + noVNC + GPU 加速
  >   - 架构: Cage → Wayland 应用 → wayvnc (硬件编码) → noVNC (WebGL 解码)
  >   - GPU 加速路径:
  >     - 应用渲染(GPU) → wayvnc 编码(VA-API/NVENC) → noVNC 解码(WebGL)
  >   - 多窗口管理: 每个应用窗口独立 wayvnc 实例
  > - 备选方案: XWayland 转发(性能较差)
  > - 高级方案: 自定义 Wayland 合成器接口(研究性质)
  >
  > #### 2. 浏览器跨域访问
  >
  > - **解决方案**: Rust 反向代理服务
  >   - 访问模式: `http://localhost:<port>/proxy?url=<encoded_target_url>`
  >   - 安全措施: 内容重写(CSP/HSTS)、请求过滤
  >
  > #### 3. 网页应用仓库系统
  >
  > - **仓库结构**:
  >   - 本地仓库: `~/.local/share/pageos/software/<user>/<repo>/<version>`
  >   - 在线仓库: HTTPS 访问
  > - **应用包格式**:
  >
  >   ```json
  >   // metadata.json示例
  >   {
  >     "name": "设置管理器",
  >     "id": "pageos.settings-manager",
  >     "version": "1.0.0",
  >     "type": "system",
  >     "permissions": ["system:reboot", "fs:read:/etc"],
  >     "entry": "index.html"
  >   }
  >   ```
  >
  > - **权限系统**:
  >   - 权限粒度: `system:reboot`, `fs:write:/path`, `network:external`
  >   - 授权流程: 安装时审批+运行时检查
  >
  > #### 4. 核心挑战解决方案
  >
  > - **应用认证**: URL 令牌注入(`app_id`+安全令牌)
  > - **安全加固**:
  >   - 应用包签名(ECDSA/RSA)
  >   - 强制 HTTPS/WSS
  >   - 严格 CSP 策略
  >   - iframe 沙盒属性
  > - **数据存储**:
  >   - 简单数据: localStorage
  >   - 重要数据: Rust 服务端 API(`/fs/read`, `/fs/write`)
  >
  > #### 5. 实施路线图
  >
  > 1. 定义`metadata.json`规范
  > 2. 实现核心仓库服务(安装/更新/权限)
  > 3. 解决应用认证问题(URL 令牌)
  > 4. 开发基础应用启动器

## 方案

### 仓库

#### swaybien/pageos-greet（已完成）

Rust 语言编写的登录界面服务器。包含一个最小网页服务器和内置的登录页面。

以下是除帮助参数外可用参数：

```bash
pageos-greet \
  --port 12801 \
  --page /path/to/login.html \
  --launch-command "cage -s -- firefox --kiosk --no-remote http://127.0.0.1:12801" \
  --session-command "pageos-core -p 12800 --command \"cage -s -- firefox --kiosk --no-remote http://127.0.0.1:12800\""
```

- 登录界面：
  - 显示管理器采用 `greetd`；
  - 默认的 HTML 登录页面内置在主程序 `pageos-greet` 中；
  - 如果需要自定义登录界面，则使用参数传入自定义 HTML；

#### swaybien/pageos-core（当前目标）

Rust 语言编写的 Web-Centric OS Framework（双向服务端程序）。

- 网页服务器（已完成）：只是在将 `$HOME/.local/share/pageos/` 下的文件（如 index.html ）读取并显示。
- 网页应用仓库：
  - 本地网页应用仓库：`$HOME/.local/share/pageos/software`
  - 在线网页应用仓库：`https://<domain>/software`
- 用户界面：
  - 启动用户界面的命令：`pageos-core -p 12800 --command "cage -s -- firefox --kiosk --no-remote http://127.0.0.1:12800"`；
  - 默认的 HTML 用户界面内置在主程序 `pageos-core` 中；
  - 如果需要自定义用户界面，则将自定义 HTML 写入 `$HOME/.local/share/pageos/index.html`，`pageos-core` 会自动读取并优先使用这个文件；
- 存储：
  - 在统一的 websocket 接口实现连接后可用类似 localstorage 的方法存取数据到本地文件；
  - 在统一的 websocket 接口实现连接后可用类似 indexdb 的方法存取数据到本地文件；
- 对接 polkit：
  - 当需要授权时，通知网页，并接受网页传回的验证信息；
- 系统调用 Linux 命令交互：
  - 在统一的 websocket 接口实现执行命令并实时返回结果（以给 JS 处理）；

#### swaybien/pageos-apps

官方的网页应用仓库。托管登陆页面、用户界面和依赖的软件包。

#### swaybien/pageos（已完成）

基于 Arch Linux 的 PageOS 镜像维护仓库。

#### swaybien/pageos-pkgr

Rust 语言编写的网页应用仓库管理程序。

- 创建新的软件源仓库
- 更新当前软件源索引
- 从其它软件源同步

#### swaybien/pageos-pkgr-ui

网页应用仓库管理程序 GUI。
