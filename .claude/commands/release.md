---
description: developからrelease/vX.Y.Zブランチを作成し、リリースフローを開始します。
tags: [project]
---

# リリースコマンド

GitHub Actionsワークフローを起動して、developブランチから`release/vX.Y.Z`ブランチを自動作成し、正式リリースフローを開始します。

## 実行内容

1. GitHub CLIで`create-release.yml`ワークフローを起動（developブランチを指定）
2. GitHub Actions側で以下を自動実行：
   - developブランチでsemantic-releaseのドライランを実行
   - 次バージョン番号を決定
   - `release/vX.Y.Z`ブランチを作成してpush
3. releaseブランチのpushを契機に`release.yml`が起動：
   - semantic-releaseによりCHANGELOG/Cargo.toml/タグ/GitHub Releaseを更新
   - releaseブランチをmainへ直接取り込み（バックマージでdevelopも同期）
4. mainへのマージ後、`publish.yml`が起動：
   - `release-binaries.yml`を呼び出して各プラットフォーム向けバイナリを添付

## 前提条件

- GitHub CLIが認証済みであること（`gh auth status`）
- developブランチにConventional Commits準拠のコミットがあること

## 使用方法

以下のスクリプトを実行してリリースフローを開始します：

```bash
scripts/create-release-branch.sh
```

スクリプトは`gh workflow run create-release.yml --ref develop`を実行し、GitHub Actionsワークフローを起動します。その後、create-release.yml → release.yml → publish.yml → release-binaries.yml が連鎖的に実行され、リリースが完了します。
