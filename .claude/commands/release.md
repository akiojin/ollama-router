正式リリースプロセスを開始します。

## 概要

このコマンドは、developブランチからmainブランチへのプルリクエストを作成し、
正式版リリースプロセスを開始します。

## 実行内容

1. **前提条件チェック**
   - GitHub CLI (gh)の確認
   - develop/mainブランチの存在確認
   - 既存PRの確認

2. **PR作成**
   - develop → main プルリクエストを自動作成
   - リリース用テンプレートを適用
   - `release`, `auto-merge` ラベルを付与

3. **自動処理**（PR作成後）
   - 品質チェックの自動実行
   - 品質チェック合格後、mainへ自動マージ
   - semantic-releaseによるバージョニング
   - CHANGELOG・Cargo.toml自動更新
   - GitHubタグ・リリース自動作成
   - 全プラットフォームのバイナリ自動ビルド

## 使用方法

単に以下のコマンドを実行してください：

```bash
./scripts/release/create-release-pr.sh
```

または、Claude Codeから：

```
/release
```

## 注意事項

- このコマンドはPR作成のみを行います
- 実際のマージとリリースは品質チェック合格後に自動実行されます
- developブランチが事前に作成されている必要があります

## トラブルシューティング

### developブランチが存在しない

```bash
# developブランチを作成（メンテナが実行）
git checkout -b develop main
git push -u origin develop
```

### 既存のPRがある場合

スクリプトが既存PRを検出し、更新するか確認します。

---

実行しますか？ (このプロンプトを確認後、スクリプトを実行します)
