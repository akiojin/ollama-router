# gptossアーキテクチャエイリアスサポート

## 概要

llama.cppが`gptoss`アーキテクチャ名（ハイフンなし）を認識できるようにする。

## ビジネス価値

- Ollamaでプルしたgpt-ossモデルをC++ Nodeで使用できる
- ユーザーがモデル変換等の追加作業なしで推論を実行できる

## ユーザーストーリー

### US-1: gpt-ossモデルを使用したい

ユーザーとして、`ollama pull gpt-oss:20b`でダウンロードしたモデルを
C++ Nodeでそのまま使用できる。

**受け入れ条件**:

- GGUFメタデータの`general.architecture = "gptoss"`を認識する
- `LLM_ARCH_OPENAI_MOE`として処理される
- 既存の`gpt-oss`（ハイフン付き）も引き続きサポートする

## 機能要件

### FR-1: アーキテクチャ名エイリアス

`llm_arch_from_string`関数で以下のエイリアスをサポート:

| 入力 | 出力 |
|------|------|
| `gptoss` | `LLM_ARCH_OPENAI_MOE` |
| `gpt-oss` | `LLM_ARCH_OPENAI_MOE` |

## 非機能要件

### NFR-1: 後方互換性

- 既存の`gpt-oss`（ハイフン付き）は引き続き動作する
- 他のアーキテクチャへの影響なし

## 技術的背景

### 問題の原因

Ollamaが生成するGGUFファイルは`general.architecture = "gptoss"`を使用するが、
llama.cppのマッピングは`"gpt-oss"`（ハイフン付き）のみを認識する。

```cpp
// llama-arch.cpp 99行目
{ LLM_ARCH_OPENAI_MOE,       "gpt-oss"          },
```

`llm_arch_from_string("gptoss")`がマップ内の`"gpt-oss"`と一致しないため、
`LLM_ARCH_UNKNOWN`が返され、モデル読み込みが失敗する。
