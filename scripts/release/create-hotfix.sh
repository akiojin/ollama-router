#!/usr/bin/env bash

# create-hotfix.sh
# ホットフィックスブランチ作成スクリプト

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# エラーハンドラ
error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# ヘルプ表示
show_help() {
    cat <<EOF
使用方法: $0 [ISSUE_ID]

ホットフィックス用ブランチを作成します。

引数:
    ISSUE_ID    GitHub Issue番号（オプション）

例:
    $0              # ブランチ名を対話式で入力
    $0 42           # hotfix/42 を作成

説明:
    このスクリプトは本番環境の緊急修正用ブランチを作成します。
    mainブランチから分岐し、修正後はmainへマージしてパッチリリースを行います。

EOF
}

# 前提条件チェック
check_prerequisites() {
    info "前提条件をチェック中..."

    # Gitリポジトリの確認
    if ! git rev-parse --is-inside-work-tree &> /dev/null; then
        error "Gitリポジトリ内で実行してください"
    fi

    # mainブランチの存在確認
    if ! git rev-parse --verify main &> /dev/null; then
        error "mainブランチが存在しません"
    fi

    # 作業ツリーのクリーン確認
    if ! git diff-index --quiet HEAD --; then
        warning "作業ツリーに未コミットの変更があります"
        echo ""
        read -p "続行しますか? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "処理を中止しました"
        fi
    fi

    success "前提条件チェック完了"
}

# ブランチ名の決定
determine_branch_name() {
    if [ -n "${1:-}" ]; then
        BRANCH_NAME="hotfix/$1"
        info "ブランチ名: $BRANCH_NAME"
    else
        echo ""
        echo "ホットフィックスの説明を入力してください（例: fix-critical-bug, issue-42）:"
        read -r HOTFIX_DESC

        if [ -z "$HOTFIX_DESC" ]; then
            error "説明が入力されませんでした"
        fi

        BRANCH_NAME="hotfix/$HOTFIX_DESC"
        info "ブランチ名: $BRANCH_NAME"
    fi

    # ブランチ名の検証
    if git rev-parse --verify "$BRANCH_NAME" &> /dev/null; then
        error "ブランチ $BRANCH_NAME は既に存在します"
    fi
}

# ブランチ作成
create_branch() {
    info "mainブランチから $BRANCH_NAME を作成中..."

    git fetch origin main:main || warning "mainブランチの同期に失敗"
    git checkout -b "$BRANCH_NAME" origin/main || error "ブランチ作成に失敗"

    success "ブランチ作成完了: $BRANCH_NAME"
}

# 修正ガイド表示
show_guide() {
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    info "🔧 ホットフィックス作業ガイド"
    echo ""
    echo "現在のブランチ: ${GREEN}$BRANCH_NAME${NC}"
    echo ""
    echo "次のステップ:"
    echo ""
    echo "  1. 緊急修正を実装してコミット"
    echo "     ${BLUE}git add .${NC}"
    echo "     ${BLUE}git commit -m \"fix: 緊急修正の説明\"${NC}"
    echo ""
    echo "  2. ローカル品質チェックを実行"
    echo "     ${BLUE}make quality-checks${NC}"
    echo ""
    echo "  3. リモートにプッシュ"
    echo "     ${BLUE}git push -u origin $BRANCH_NAME${NC}"
    echo ""
    echo "  4. main へのPR作成"
    echo "     ${BLUE}gh pr create --base main --head $BRANCH_NAME \\${NC}"
    echo "     ${BLUE}  --title \"fix: 緊急修正の説明\" \\${NC}"
    echo "     ${BLUE}  --label \"hotfix,auto-merge\"${NC}"
    echo ""
    echo "  5. 品質チェック合格後、自動マージされます"
    echo "  6. マージ後、パッチバージョンが自動リリースされます"
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# メイン処理
main() {
    if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
        show_help
        exit 0
    fi

    cd "$PROJECT_ROOT"

    echo ""
    info "🚨 ホットフィックスブランチ作成スクリプト"
    echo ""

    check_prerequisites
    determine_branch_name "${1:-}"

    echo ""
    warning "⚠️  注意: このブランチはmainから分岐します"
    warning "⚠️  緊急修正以外の変更は含めないでください"
    echo ""
    read -p "続行しますか? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        error "処理を中止しました"
    fi

    create_branch
    show_guide

    success "✅ ホットフィックスブランチが準備できました"
}

main "$@"
