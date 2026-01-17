# 独自言語を作りたい

## 言語名を決めよう

- cursed_language .cl
- cancer .ca
- leo .le
- helianthus .he
- topazus .to

言語名:topazus
由来:11月の誕生石のラテン語名
拡張子:*.to
これに決定

## 言語の構文を決めよう

- 目標　Lisp*c++ 
- 逆張りで全部イギリス英語
- エラー文を皮肉たっぷり
- 名前空間あり
- 標準でスターリンソートがある


are you a f**kin dounut?

# 言語仕様
関数の記述方法
(funcname arg)

(let [type] name <- data) => 変数宣言
([type] [type variable] -> | process |) => 無名関数
(let [fn] name <- [type] [type var] -> | process |) => 関数作成

(let [fn] sqr <- [int] [int x] -> |(* x x)|)
(sqr 3) => 9

標準入出力
(stream::print msg)
(stream::get -> input)

名前空間streamの機能
stream::print 引数を標準出力ストリームに出力
stream::println 引数を標準出力ストリームに出力し改行する
stream::get　引数に指定した変数に表示入力ストリームから受け取って代入
stream::
