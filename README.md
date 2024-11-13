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
   .
   ├── .devcontainer
   │   └── devcontainer.json
   ├── .vscode
   │   └── settings.json
   ├── projects
   │   ├── Cargo.toml      # ワークスペースのルート
   │   ├── hello_world     # プロジェクト1
   │   │   ├── Cargo.toml
   │   │   └── src
   │   │       └── main.rs
   │   └── count          # プロジェクト2
   │       ├── Cargo.toml
   │       └── src
   │           └── main.rs
   ```
   - 開発環境の設定を分離
   - メンテナンス性の向上
   - チーム間での共有が容易

4. **Rustイメージの選択**
   - 公式イメージを使用
   - 安定性とセキュリティの確保
   - 必要な開発ツールが事前インストール済み

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

## セットアップ

### 必要なファイル

1. **.devcontainer/devcontainer.json**
```json
{
    "name": "Rust Development",
    "dockerComposeFile": "../compose.yaml",
    "service": "rust",
    "workspaceFolder": "/workspace",
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "vadimcn.vscode-lldb",
                "tamasfe.even-better-toml",
                "serayuzgur.crates",
                "ms-azuretools.vscode-docker",
                "ms-vscode-remote.remote-containers"
            ],
            "settings": {
                "terminal.integrated.defaultProfile.linux": "bash",
                "terminal.integrated.profiles.linux": {
                    "bash": {
                        "path": "/bin/bash"
                    }
                },
                "[rust]": {
                    "editor.formatOnSave": true,
                    "editor.defaultFormatter": "rust-lang.rust-analyzer"
                },
                "rust-analyzer.checkOnSave.command": "clippy",
                "rust-analyzer.cargo.allFeatures": true,
                "rust-analyzer.procMacro.enable": true,
                "docker.enableExtensionSurvey": false,
                "docker.showStartPage": false,
                "docker.images.sortBy": "CreatedTime",
                "docker.containers.sortBy": "Status",
                "remote.containers.defaultExtensions": [
                    "ms-azuretools.vscode-docker"
                ]
            }
        }
    },
    "remoteUser": "rustdev",
    "postCreateCommand": "rustup component add rust-src rustfmt clippy rust-analyzer",
    "workspaceMount": "source=${localWorkspaceFolder},target=/workspace,type=bind,consistency=cached"
}
```

2. **projects/Cargo.toml**
```toml
[workspace]
resolver = "2"
members = [
    "hello_world",
    "count"
    ## ここに新たなファイルを作成したら追加する
]
exclude = [
    "target"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
```

## 開発環境

### 新しいプロジェクトの作成

```bash
cd /workspace/projects
cargo new プロジェクト名
```

その後、`projects/Cargo.toml`の`members`配列に新しいプロジェクト名を追加します。

### rust-analyzerの設定

rust-analyzerを正しく動作させるために、以下の点に注意してください：

1. ワークスペースのルートに`Cargo.toml`が必要
2. 各プロジェクトが適切なディレクトリ構造を持つ
3. 依存関係が正しく解決される

### Docker拡張機能の機能

1. **コンテナ管理**
   - コンテナの起動/停止
   - ログの表示
   - シェルへのアクセス

2. **イメージ管理**
   - イメージの表示と管理
   - ビルド履歴の確認
   - キャッシュの管理

3. **開発サポート**
   - Dockerfile構文のハイライト
   - Docker Compose設定のバリデーション
   - コマンド補完

## トラブルシューティング

### よくある問題と解決方法

1. **rust-analyzerが動作しない場合**
   - VSCodeを再起動
   - コンテナを再ビルド
   - `cargo check`を実行して依存関係を確認

2. **ビルドエラーが発生する場合**
   - `cargo clean`を実行してキャッシュをクリア
   - プロジェクトの依存関係を確認
   - Rustのツールチェーンが正しくインストールされているか確認

3. **パスの解決に問題がある場合**
   - 絶対パスではなく相対パスを使用
   - ワークスペースのルートからの正しいパスを確認
   - マウントポイントが正しく設定されているか確認

4. **Docker関連の問題**
   - コンテナの再ビルド: `Docker: Rebuild Container`
   - 拡張機能の再インストール
   - Docker Desktopの再起動

### デバッグ方法

1. **VSCode デバッガーの使用**
   - ブレークポイントの設定
   - 変数の監視
   - コールスタックの確認

2. **ログの確認**
   - Dockerコンテナのログ
   - rust-analyzerのログ
   - Cargoのビルドログ

## 参考リンク

- [Rust Documentation](https://www.rust-lang.org/learn)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers)
- [rust-analyzer Manual](https://rust-analyzer.github.io/manual.html)
- [Docker Extension for VS Code](https://marketplace.visualstudio.com/items?itemName=ms-azuretools.vscode-docker)