<p align="center">
  <a href="https://raw.githubusercontent.com/Curzyori/anime-cursor/main/images/demo.mp4">▶️ 观看演示视频</a>
</p>

<p align="center"><img src="images/logo.svg" width="120" alt="Anime Cursor 标志" /></p>

<h1 align="center">Anime Cursor</h1>
<p align="center">
  <strong>跨平台桌面动画光标管理器</strong>
</p>

<div align="center">

  <a href="https://github.com/Curzyori/anime-cursor"><img src="https://img.shields.io/github/stars/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Stars" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/network/members"><img src="https://img.shields.io/github/forks/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Forks" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=for-the-badge&color=374151" alt="License" /></a>
  <img src="https://img.shields.io/badge/platform-win|linux-blue?style=for-the-badge" alt="Platform" />

</div>

<p align="center">
  <a href="#why">为什么</a> ·
  <a href="#key-features">功能</a> ·
  <a href="#tech-stack">技术栈</a> ·
  <a href="#architecture">架构</a> ·
  <a href="#quick-start">快速开始</a> ·
  <a href="#installation">安装</a> ·
  <a href="#preview">预览</a> ·
  <a href="#support">支持</a> ·
  <a href="#license">许可证</a>
</p>

<p align="center">
  🌐 支持 4+ 种语言 -
  <a href="README.md">🇺🇸 EN</a> ·
  <a href="README_ID.md">🇮🇩 ID</a> ·
  <a href="README_CN.md"><b>🇨🇳 CN</b></a> ·
  <a href="README_JP.md">🇯🇵 JP</a>
</p>

---

## <a id="why"></a>🕒 为什么选择 Anime Cursor？

厌倦了进入控制面板或编辑注册表才能尝试动漫光标包？Anime Cursor 是一款跨平台桌面应用，让浏览、导入和应用动画光标 (.ani) 包变得极其简单--无需寻找设置面板或输入终端命令。

|                              |                                                              |
| ----------------------------- | ------------------------------------------------------------ |
| ✅ **一键应用**              | 自动检测操作系统--无需手动选择                               |
| ✅ **包管理**                | 一次性导入含多个光标变体的 ZIP 包                            |
| ✅ **自动备份与恢复**        | 每次应用前自动备份光标--随时安全恢复                         |
| ✅ **完全离线**              | 零网络请求，无遥测--绝对隐私                                 |
| ✅ **跨平台**                | Windows 10+ 和 Linux (GNOME, KDE, XFCE)                      |

## <a id="key-features"></a>🎯 主要功能

| 功能 | 状态 | 描述 |
| :--- | :---: | :--- |
| **导入包** | ✅ | 拖放 ZIP 或单个 .ani - 自动解压和解析 |
| **仪表盘网格** | ✅ | 浏览所有已安装包，支持按名称/日期排序 |
| **包详情视图** | ✅ | 每个变体的预览及应用按钮 |
| **一键应用** | ✅ | 自动检测操作系统 - Windows 注册表或 Linux Xcursor |
| **自动备份** | ✅ | 每次应用前保存当前光标 |
| **恢复默认** | ✅ | 从设置中恢复为操作系统默认光标 |
| **多语言界面** | ✅ | 一键切换 EN, ID, CN, JP |
| **深色/浅色主题** | ✅ | 持久保存的主题切换 |
| **可折叠侧边栏** | ✅ | 仅图标的紧凑模式 |

## <a id="tech-stack"></a>🛠️ 技术栈

- **桌面框架:** Tauri v2 (Rust + webview)
- **前端:** React 18 + TypeScript + Tailwind CSS
- **状态管理:** Zustand (应用状态) + Tauri IPC (异步命令)
- **构建:** Vite 6
- **后端:** Rust (serde, zip, winreg, xcursor)
- **格式:** .ani RIFF 动画光标 + .zip 包

## <a id="architecture"></a>🏗️ 架构

```
anime-cursor/
├── src-tauri/           # Rust 后端
│   ├── src/
│   │   ├── main.rs      # Tauri 入口，命令
│   │   ├── parser.rs    # .ANI RIFF 解析器
│   │   ├── applier.rs   # 光标应用/恢复 (Win + Linux)
│   │   ├── pack.rs      # ZIP 解压，元数据读取
│   │   └── backup.rs    # 光标备份管理
│   ├── icons/           # 各平台应用图标
│   └── Cargo.toml
├── src/                 # React 前端
│   ├── App.tsx
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Import.tsx
│   │   ├── PackDetail.tsx
│   │   └── Settings.tsx
│   ├── components/
│   ├── i18n/            # EN, ID, CN, JP 语言文件
│   ├── hooks/
│   ├── styles/
│   └── types/
├── public/
│   └── logo.svg
├── .example/            # 参考光标包用于测试
├── package.json
└── tauri.conf.json
```

## <a id="quick-start"></a>🚀 快速开始

下载最新版本（推荐）：

<a href="https://github.com/Curzyori/anime-cursor/releases">下载 Anime Cursor →</a>

支持 .deb (Debian/Ubuntu)，.rpm (Fedora)，和 .AppImage (全部 Linux)。

## <a id="installation"></a>📦 安装

从源码构建 (Node 18+)：

```bash
git clone https://github.com/Curzyori/anime-cursor.git
cd anime-cursor
npm install
npm run tauri build
```

构建产物在 `src-tauri/target/release/anime-cursor`。

系统依赖 (Linux)：

```bash
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev
```

## <a id="preview"></a>🖼️ 预览

<div align="center">

| 仪表盘 | 包详情 |
|-----------|-------------|
| <img src="images/dashboard.png" width="250" alt="仪表盘" /> | <img src="images/pack-detail.png" width="250" alt="包详情" /> |
| 导入 | 设置 |
| <img src="images/import.png" width="250" alt="导入" /> | <img src="images/settings.png" width="250" alt="设置" /> |

</div>

> 占位截图--请将您的截图添加到 `images/` 文件夹。

## <a id="support"></a>☕ 支持

如果你觉得这个项目对你有帮助，请考虑给一个 ⭐ Star 或 🍴 Fork 以示支持，这能让我更有动力继续开发更多有趣的开源项目！每一个 Star 和 Fork 对开发者来说都无比珍贵。

您的捐赠能让这个项目保持免费与开源。每一份贡献都至关重要，您的支持也将激励我未来继续开发更多有趣的开源项目。

<a href="https://donate.curzy.dev/">请我喝杯咖啡吧！ 💝</a>

<a href="https://donate.curzy.dev/">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" width="200" />
</a>

## <a id="license"></a>⚖️ 许可证

GPL-3.0 - 详见 <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE">LICENSE</a>。

<p align="center">
  <sub>作为 50 Projects Challenge 的第 22 个项目，由 **@Curzyori** 用心打造</sub>
</p>
