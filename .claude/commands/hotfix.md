緊急修正（ホットフィックス）プロセスを開始します。

## 概要

本番環境で発見された重大なバグを迅速に修正するための
ホットフィックスブランチを作成します。

## 実行内容

1. **前提条件チェック**
   - Gitリポジトリの確認
   - mainブランチの存在確認
   - 作業ツリーのクリーン確認

2. **ブランチ作成**
   - mainブランチから hotfix/* ブランチを作成
   - 修正作業のガイドを表示

3. **修正後の流れ**（ガイド表示）
   - 修正実装とコミット
   - ローカル品質チェック実行
   - リモートへプッシュ
   - main へのPR作成
   - 品質チェック合格後、自動マージ
   - パッチバージョンの自動リリース

## 使用方法

### オプション1: Issue番号を指定

```bash
./scripts/release/create-hotfix.sh 42
# → hotfix/42 ブランチを作成
```

### オプション2: 説明を指定

```bash
./scripts/release/create-hotfix.sh fix-critical-bug
# → hotfix/fix-critical-bug ブランチを作成
```

### オプション3: 対話式

```bash
./scripts/release/create-hotfix.sh
# → プロンプトで説明を入力
```

または、Claude Codeから：

```
/hotfix
```

## コミットメッセージの例

```
fix: メモリリークを修正

本番環境で発見されたメモリリークを修正しました。
長時間実行時にメモリ使用量が増加し続ける問題を解決します。
```

## 注意事項

- ホットフィックスはmainブランチから分岐します
- 緊急修正以外の変更は含めないでください
- Conventional Commits形式でコミットしてください（`fix:` プレフィックス）
- 修正後は必ずローカル品質チェックを実行してください

## 修正作業の流れ

1. **ブランチ作成**（このコマンド）
   ```bash
   /hotfix
   ```

2. **修正実装**
   ```bash
   # コードを修正
   git add .
   git commit -m "fix: 緊急修正の説明"
   ```

3. **品質チェック**
   ```bash
   make quality-checks
   ```

4. **プッシュとPR作成**
   ```bash
   git push -u origin hotfix/xxx
   gh pr create --base main --head hotfix/xxx \
     --title "fix: 緊急修正の説明" \
     --label "hotfix,auto-merge"
   ```

5. **自動マージとリリース**
   - 品質チェック合格後、自動マージ
   - パッチバージョン（例: v1.2.4）が自動リリース

---

実行しますか？ (このプロンプトを確認後、スクリプトを実行します)
