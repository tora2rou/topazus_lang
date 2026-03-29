# 自作関数型言語：ParserからLLVM実装までのロードマップ

## 1. Parser（構文解析）フェーズ
解析のゴール：トークン列を木構造（AST）に変換する

### [ ] AST（抽象構文木）のデータ構造定義
- `Literal`: 数値 (`i64`, `f64`)、文字列
- `Identifier`: 変数名
- `Let`: `let [type] name <- value in body`
- `Apply`: 関数呼び出し `(f x y)`
- `Lambda`: 匿名関数 `|x| -> { expr }`
- `If`: `if cond then else`

### [ ] 再帰下降パーサーの実装
- `parse_expression`: 全てのパースの入り口（トークンを見て分岐）
- `parse_primary`: 括弧、リテラル、識別子の処理
- `parse_apply`: 関数適用のパース（左結合のループ処理）
- `parse_let`: 予約語 `let` から `in` までの構造解析
- `expect(token)` / `consume()`: 期待するトークンのチェックと消費ユーティリティ

---

## 2. セマンティクス（意味解析）フェーズ
解析のゴール：ASTが言語的に正しいか検証し、情報を付加する

### [ ] 名前解決（Name Resolution）
- 識別子がどの定義（引数やLet）を指しているか紐付け
- スコープ（環境）のネスト構造の管理

### [ ] 型推論（Type Inference）
- Hindley-Milnerアルゴリズム等の実装
- 各ASTノードへの具体的な型情報の付与（LLVM生成時に必須）

---

## 3. 中間変換フェーズ
解析のゴール：関数型の概念をLLVM（命令型）に変換可能な形にする

### [ ] クロージャ変換（Closure Conversion）
- 外部変数を参照する関数を「関数ポインタ + 環境ポインタ」のペアに変換
- LLVMの関数はトップレベルしか存在しないため、ネストした関数をフラット化

---

## 4. LLVM IR生成（Codegen）フェーズ
解析のゴール：LLVM APIを使用して中間表現を出力する

### [ ] LLVM基礎構造のセットアップ
- `llvm::Context`: 型や定数の管理
- `llvm::Module`: ファイル単位のコンテナ
- `llvm::IRBuilder`: 命令の生成器

### [ ] 各ノードのCodegen関数実装
- `gen_literal()`: `ConstantInt::get` 等で定数生成
- `gen_variable()`: シンボルテーブルから `Value*` をロード
- `gen_apply()`: `Builder.CreateCall` による関数呼び出し
- `gen_let()`: `alloca` によるメモリ割り当てとシンボル登録
- `gen_function()`: `llvm::Function` の作成と基本ブロックの挿入

---

## 5. 最適化・実行フェーズ
解析のゴール：プログラムを実行可能な形にする

### [ ] 最適化（Optimization）
- `PassBuilder` を使用した標準的な最適化パスの適用

### [ ] 実行環境の構築
- JIT (Just-In-Time): `LLVM ORC JIT` を使用した即時実行
- AOT (Ahead-Of-Time): オブジェクトファイル (`.o`) の出力とリンク
