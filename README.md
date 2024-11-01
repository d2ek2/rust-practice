# Rust Development Environment with Docker

Docker と VS Code Dev Containers を使用した、Mac最適化のRust開発環境です。

## 📋 目次

- [環境構成](#環境構成)
- [構成の説明](#構成の説明)
- [Dev Containersの使用方法](#dev-containersの使用方法)
- [セットアップ](#セットアップ)
- [使い方](#使い方)
- [開発環境](#開発環境)
- [トラブルシューティング](#トラブルシューティング)

## 環境構成

- Rust 1.74
- Docker
- VS Code + Dev Containers
- Git

## 構成の説明

### なぜこの構成を選んだのか

1. **Dockerの採用理由**
   - ホストマシンを汚さない開発環境
   - チーム間で同一環境を共有可能
   - M1/M2/M3 Macでのクロスプラットフォーム対応
   - 依存関係の完全な分離

2. **Dev Containersを選んだ理由**
   - VSCodeとの完全な統合
   - コンテナ内でのスムーズな開発体験
   - 拡張機能の自動インストール
   - 設定の共有が容易

3. **ファイル構成の意図**
   ```
   .devcontainer/
   ├── devcontainer.json  # VS Code設定
   └── Dockerfile         # 環境定義
   ```
   - 開発環境の設定を分離
   - メンテナンス性の向上
   - チーム間での共有が容易

4. **Alpine LinuxベースのDockerイメージ**
   - 軽量で高速な環境
   - セキュリティリスクの最小化
   - ビルド時間の短縮

## Dev Containersの使用方法

### 基本的な使い方

1. **初回セットアップ**
   - VS Codeで「Dev Containers」拡張機能をインストール
   - プロジェクトフォルダを開く
   - 左下の「><」アイコンをクリック
   - 「Reopen in Container」を選択

2. **開発環境の起動**
   ```bash
   # プロジェクトディレクトリで
   code .
   ```
   - コマンドパレット（Cmd + Shift + P）を開く
   - 「Dev Containers: Reopen in Container」を実行

3. **コンテナ内での作業**
   - ターミナル: ``` Ctrl + ` ```
   - ファイル編集: 通常のVS Code同様
   - Git操作: 内蔵のGitツール使用可能

### 高度な使用方法

1. **カスタム設定**
   ```json
   // devcontainer.jsonの例
   {
     "customizations": {
       "vscode": {
         "settings": {
           "editor.formatOnSave": true,
           "rust-analyzer.checkOnSave.command": "clippy"
         }
       }
     }
   }
   ```

2. **拡張機能の自動インストール**
   - rust-analyzer
   - LLDB
   - Git Lens
   - その他必要な拡張機能

3. **デバッグ機能の使用**
   - ブレークポイントの設定
   - 変数の監視
   - ステップ実行

4. **ポートフォワーディング**
   - 自動的にポートを転送
   - Web開発時に便利
   - カスタムポートの設定可能

### 便利な機能

1. **ファイル同期**
   - ホストとコンテナ間でリアルタイム同期
   - 変更が即座に反映

2. **統合ターミナル**
   - コンテナ内のシェルに直接アクセス
   - 複数ターミナルの同時使用可能

3. **Git操作**
   - コミット、プッシュ、プル
   - ブランチ管理
   - 差分表示

4. **タスク実行**
   - ビルド
   - テスト
   - リント
   - フォーマット

[以下、既存のセクションが続きます...]