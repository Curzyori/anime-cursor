<p align="center">
  <a href="https://raw.githubusercontent.com/Curzyori/anime-cursor/main/images/demo.mp4">▶️ デモ動画を見る</a>
</p>

https://github.com/user-attachments/assets/6bcdc8f4-6b75-4bfb-a7d0-7de54b747782

<p align="center"><img src="images/logo.svg" width="120" alt="Anime Cursor ロゴ" /></p>

<h1 align="center">Anime Cursor</h1>
<p align="center">
  <strong>クロスプラットフォーム デスクトップ アニメーションカーソルマネージャー</strong>
</p>

<div align="center">

  <a href="https://github.com/Curzyori/anime-cursor"><img src="https://img.shields.io/github/stars/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Stars" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/network/members"><img src="https://img.shields.io/github/forks/Curzyori/anime-cursor?style=for-the-badge&color=374151" alt="Forks" /></a>
  <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=for-the-badge&color=374151" alt="License" /></a>
  <img src="https://img.shields.io/badge/platform-win|linux-blue?style=for-the-badge" alt="Platform" />

</div>

<p align="center">
  <a href="#why">なぜ</a> ·
  <a href="#key-features">機能</a> ·
  <a href="#tech-stack">技術スタック</a> ·
  <a href="#architecture">アーキテクチャ</a> ·
  <a href="#quick-start">クイックスタート</a> ·
  <a href="#installation">インストール</a> ·
  <a href="#preview">プレビュー</a> ·
  <a href="#support">サポート</a> ·
  <a href="#license">ライセンス</a>
</p>

<p align="center">
  🌐 4+ 言語に対応 -
  <a href="README.md">🇺🇸 EN</a> ·
  <a href="README_ID.md">🇮🇩 ID</a> ·
  <a href="README_CN.md">🇨🇳 CN</a> ·
  <a href="README_JP.md"><b>🇯🇵 JP</b></a>
</p>

---

## <a id="why"></a>🕒 なぜ Anime Cursor なのか？

コントロールパネルを開いたりレジストリを編集したりするのにうんざりしていませんか？Anime Cursor は、アニメーションカーソル (.ani) パックの閲覧、インポート、適用を驚くほど簡単にするクロスプラットフォームデスクトップアプリです - 設定パネルを探したりターミナルコマンドを入力する必要はありません。

|                              |                                                              |
| ----------------------------- | ------------------------------------------------------------ |
| ✅ **ワンクリック適用**     | OS を自動検出 - 手動選択は不要                               |
| ✅ **パック管理**            | 複数のカーソルバリアントを含む ZIP を一度にインポート        |
| ✅ **自動バックアップ＆復元** | 適用前にカーソルを自動バックアップ - いつでも安全に復元      |
| ✅ **完全オフライン**        | ゼロネットワークリクエスト、テレメトリなし - 完全プライバシー |
| ✅ **クロスプラットフォーム** | Windows 10+ と Linux (GNOME, KDE, XFCE)                      |

## <a id="key-features"></a>🎯 主な機能

| 機能 | ステータス | 説明 |
| :--- | :---: | :--- |
| **パックインポート** | ✅ | ZIP または .ani をドラッグ＆ドロップ - 自動展開・解析 |
| **ダッシュボードグリッド** | ✅ | インストール済みパックを名前/日付でソートして表示 |
| **パック詳細ビュー** | ✅ | バリアントごとのプレビューと適用ボタン |
| **ワンクリック適用** | ✅ | OS 自動検出 - Windows レジストリまたは Linux Xcursor |
| **自動バックアップ** | ✅ | 適用前に現在のカーソルを保存 |
| **デフォルトに戻す** | ✅ | 設定から OS デフォルトカーソルに復元 |
| **多言語 UI** | ✅ | ワンクリックで EN, ID, CN, JP を切り替え |
| **ダーク/ライトテーマ** | ✅ | 保存可能なテーマ切り替え |
| **折りたたみサイドバー** | ✅ | アイコンのみのコンパクトモード |

## <a id="tech-stack"></a>🛠️ 技術スタック

- **デスクトップフレームワーク:** Tauri v2 (Rust + webview)
- **フロントエンド:** React 18 + TypeScript + Tailwind CSS
- **状態管理:** Zustand (アプリ状態) + Tauri IPC (非同期コマンド)
- **ビルド:** Vite 6
- **バックエンド:** Rust (serde, zip, winreg, xcursor)
- **フォーマット:** .ani RIFF アニメーションカーソル + .zip パック

## <a id="architecture"></a>🏗️ アーキテクチャ

```
anime-cursor/
├── src-tauri/           # Rust バックエンド
│   ├── src/
│   │   ├── main.rs      # Tauri エントリ、コマンド
│   │   ├── parser.rs    # .ANI RIFF パーサー
│   │   ├── applier.rs   # カーソル適用/復元 (Win + Linux)
│   │   ├── pack.rs      # ZIP 展開、メタデータ読み取り
│   │   └── backup.rs    # カーソルバックアップ管理
│   ├── icons/           # プラットフォーム別アプリアイコン
│   └── Cargo.toml
├── src/                 # React フロントエンド
│   ├── App.tsx
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Import.tsx
│   │   ├── PackDetail.tsx
│   │   └── Settings.tsx
│   ├── components/
│   ├── i18n/            # EN, ID, CN, JP 言語ファイル
│   ├── hooks/
│   ├── styles/
│   └── types/
├── public/
│   └── logo.svg
├── .example/            # テスト用リファレンスカーソルパック
├── package.json
└── tauri.conf.json
```

## <a id="quick-start"></a>🚀 クイックスタート

最新リリースをダウンロード（推奨）：

<a href="https://github.com/Curzyori/anime-cursor/releases">Anime Cursor をダウンロード →</a>

対応形式: .deb (Debian/Ubuntu)、.rpm (Fedora)、.AppImage (全 Linux)。

## <a id="installation"></a>📦 インストール

ソースからビルド (Node 18+)：

```bash
git clone https://github.com/Curzyori/anime-cursor.git
cd anime-cursor
npm install
npm run tauri build
```

バイナリは `src-tauri/target/release/anime-cursor` に生成されます。

システム依存関係 (Linux)：

```bash
sudo apt install libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev
```

## <a id="preview"></a>🖼️ プレビュー

<div align="center">

| ダッシュボード | パック詳細 |
|-----------|-------------|
| <img src="images/dashboard.png" width="250" alt="ダッシュボード" /> | <img src="images/pack-detail.png" width="250" alt="パック詳細" /> |
| インポート | 設定 |
| <img src="images/import.png" width="250" alt="インポート" /> | <img src="images/settings.png" width="250" alt="設定" /> |

</div>

> プレースホルダースクリーンショット - `images/` フォルダにご自身のスクリーンショットを追加してください。

## <a id="support"></a>☕ サポート

このプロジェクトが役に立った場合は、⭐ Star を付けるか 🍴 Fork することを検討してください。開発の励みになり、今後より魅力的なオープンソースプロジェクトを作成するモチベーションになります！1つのスターやフォークが開発者にとって非常に価値があります。

ご寄付により、このプロジェクトを無料でオープンソースに維持できます。皆様のご支援が力になり、今後も魅力的なオープンソースプロジェクトを開発し続ける原動力となります。

<a href="https://donate.curzy.dev/">コーヒーをご馳走してください！ 💝</a>

<a href="https://donate.curzy.dev/">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" width="200" />
</a>

## <a id="license"></a>⚖️ ライセンス

GPL-3.0 - <a href="https://github.com/Curzyori/anime-cursor/blob/main/LICENSE">LICENSE</a> を参照。

<p align="center">
  <sub>50 Projects Challenge の22番目のプロジェクトとして、**@Curzyori** が情熱を込めて開発</sub>
</p>
