# 機能仕様書: 自動マージテスト

**機能ID**: `SPEC-12ab34cd`
**作成日**: 2025-10-30
**テスト目的**: T011 - 全チェック合格後の自動マージ検証

## 概要

このSPECは統合テスト用のダミーSPECです。自動マージ機能（SPEC-47c6f44c）の動作検証を目的としています。

## テストシナリオ

1. featureブランチでPRを作成
2. 全タスク完了状態（tasks.mdですべて `- [x]`）
3. Conventional Commits準拠のコミットメッセージ
4. GitHub Actions「Quality Checks」が全て成功
5. GitHub Actions「Auto Merge」が起動してPRを自動マージ

## 期待される結果

- ✅ quality-checksワークフローが成功
- ✅ auto-mergeワークフローが起動
- ✅ PRが自動的にmainブランチにマージされる
- ✅ マージコミットが作成される（MERGE method使用）
