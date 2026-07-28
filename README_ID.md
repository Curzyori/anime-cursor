<p align="center">
  <video src="images/demo.mp4" width="600" controls muted loop autoplay>
    <a href="images/demo.mp4">Tonton video demo</a>
  </video>
</p>

<p align="center"><img src="images/logo.svg" width="120" alt="Anime Cursor Logo" /></p>

<h1 align="center">Anime Cursor</h1>
<p align="center">
  <strong>Pengelola Kursor Animasi Desktop Lintas Platform</strong>
</p>

<div align="center">

  <a href="https://github.com/Curzyori/anime-cursor"><img src="https://img.shields.io/github/stars/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Stars" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/network/members"><img src="https://img.shields.io/github/forks/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Forks" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=for-the-badge&color=374151" alt="License" /></a>
  <img src="https://img.shields.io/badge/platform-win|linux-blue?style=for-the-badge" alt="Platform" />

</div>

<p align="center">
  <a href="#why">Kenapa</a> ·
  <a href="#key-features">Fitur</a> ·
  <a href="#tech-stack">Tech Stack</a> ·
  <a href="#architecture">Arsitektur</a> ·
  <a href="#quick-start">Mulai</a> ·
  <a href="#installation">Instalasi</a> ·
  <a href="#preview">Preview</a> ·
  <a href="#support">Dukung</a> ·
  <a href="#license">Lisensi</a>
</p>

<p align="center">
  🌐 Dalam 4+ bahasa -
  <a href="README.md">🇺🇸 EN</a> ·
  <a href="README_ID.md"><b>🇮🇩 ID</b></a> ·
  <a href="README_CN.md">🇨🇳 CN</a> ·
  <a href="README_JP.md">🇯🇵 JP</a>
</p>

---

## <a id="why"></a>🕒 Kenapa Anime Cursor?

Bosan harus masuk Control Panel atau edit registry cuma buat nyoba kursor anime? Anime Cursor adalah aplikasi desktop lintas platform yang bikin browsing, import, dan apply kursor animasi (.ani) jadi super gampang - tanpa perlu cari-cari pengaturan atau ngetik perintah terminal.

|                              |                                                              |
| ----------------------------- | ------------------------------------------------------------ |
| ✅ **Satu Klik Apply**       | Otomatis deteksi OS - tanpa perlu pilih manual               |
| ✅ **Manajemen Berbasis Pack** | Import ZIP dengan banyak varian kursor sekaligus              |
| ✅ **Backup & Restore Otomatis** | Backup cursor sebelum apply - aman kapan saja                |
| ✅ **Sepenuhnya Offline**     | Nol permintaan jaringan, zero telemetri - privasi mutlak      |
| ✅ **Lintas Platform**        | Windows 10+ dan Linux (GNOME, KDE, XFCE)                     |

## <a id="key-features"></a>🎯 Fitur Utama

| Fitur | Status | Deskripsi |
| :--- | :---: | :--- |
| **Import Pack** | ✅ | Drag-drop ZIP atau .ani - ekstrak dan parse otomatis |
| **Grid Dashboard** | ✅ | Lihat semua pack terinstal dengan urut berdasarkan nama/tanggal |
| **Detail Pack** | ✅ | Preview per varian dengan tombol apply |
| **Satu Klik Apply** | ✅ | Deteksi OS otomatis - registry Windows atau Xcursor Linux |
| **Backup Otomatis** | ✅ | Simpan cursor saat ini sebelum setiap apply |
| **Restore Default** | ✅ | Kembali ke cursor bawaan OS dari Settings |
| **UI Multi-Bahasa** | ✅ | EN, ID, CN, JP dengan satu klik selektor |
| **Tema Dark/Light** | ✅ | Switcher tema yang tersimpan otomatis |
| **Sidebar Kolaps** | ✅ | Mode kompak hanya ikon |

## <a id="tech-stack"></a>🛠️ Tech Stack

- **Desktop Framework:** Tauri v2 (Rust + webview)
- **Frontend:** React 18 + TypeScript + Tailwind CSS
- **State:** Zustand (app state) + Tauri IPC (async commands)
- **Build:** Vite 6
- **Backend:** Rust (serde, zip, winreg, xcursor)
- **Format:** .ani RIFF animated cursor + .zip pack

## <a id="architecture"></a>🏗️ Arsitektur

```
anime-cursor/
├── src-tauri/           # Backend Rust
│   ├── src/
│   │   ├── main.rs      # Entry Tauri, commands
│   │   ├── parser.rs    # Parser .ANI RIFF
│   │   ├── applier.rs   # Apply/restore cursor (Win + Linux)
│   │   ├── pack.rs      # Ekstrak ZIP, baca metadata
│   │   └── backup.rs    # Manajemen backup cursor
│   ├── icons/           # Ikon aplikasi per platform
│   └── Cargo.toml
├── src/                 # Frontend React
│   ├── App.tsx
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Import.tsx
│   │   ├── PackDetail.tsx
│   │   └── Settings.tsx
│   ├── components/
│   ├── i18n/            # File bahasa EN, ID, CN, JP
│   ├── hooks/
│   ├── styles/
│   └── types/
├── public/
│   └── logo.svg
├── .example/            # Pack kursor referensi untuk testing
├── package.json
└── tauri.conf.json
```

## <a id="quick-start"></a>🚀 Mulai Cepat

Unduh rilis terbaru (direkomendasikan):

<a href="https://github.com/Curzyori/anime-cursor/releases">Download Anime Cursor →</a>

Mendukung .deb (Debian/Ubuntu), .rpm (Fedora), dan .AppImage (semua Linux).

## <a id="installation"></a>📦 Instalasi

Build dari source (Node 18+):

```bash
git clone https://github.com/Curzyori/anime-cursor.git
cd anime-cursor
npm install
npm run tauri build
```

Binary ada di `src-tauri/target/release/anime-cursor`.

Dependensi sistem (Linux):

```bash
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev
```

## <a id="preview"></a>🖼️ Preview

<div align="center">

| Dashboard | Detail Pack |
|-----------|-------------|
| <img src="images/dashboard.png" width="250" alt="Dashboard" /> | <img src="images/pack-detail.png" width="250" alt="Detail Pack" /> |
| Import | Pengaturan |
| <img src="images/import.png" width="250" alt="Import" /> | <img src="images/settings.png" width="250" alt="Pengaturan" /> |

</div>


## <a id="support"></a>☕ Dukung

Jika Anda merasa project ini bermanfaat, mohon pertimbangkan untuk memberikan ⭐ Star atau melakukan 🍴 Fork untuk menunjukkan dukungan dan membuat saya lebih semangat lagi membuat project open-source yang menarik ke depannya! Setiap star dan fork sangat berharga bagi developer.

Donasi Anda menjaga proyek ini tetap gratis dan open-source. Setiap kontribusi sangat berarti, dan dukungan Anda membantu saya untuk terus membuat proyek open-source menarik ke depannya.

<a href="https://donate.curzy.dev/">Dukung project ini dengan membelikan saya kopi! 💝</a>

<a href="https://donate.curzy.dev/">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" width="200" />
</a>

## <a id="license"></a>⚖️ Lisensi

GPL-3.0 - lihat <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE">LICENSE</a>.

<p align="center">
  <sub>Dibuat dengan dedikasi sebagai project ke-22 dari 50 Projects Challenge oleh **@Curzyori**</sub>
</p>
