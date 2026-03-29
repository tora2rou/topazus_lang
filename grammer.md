# Topazus Language grammer
## 概要
### topazusについて
lisp風の記法で記述するプログラミング言語
### パラダイム
関数型言語
### インストール方法
出来てから考える
### コンパイラ
rust製
### hello world
まずはhello,worldの出力.以下のようにして行う.\
`(println "Hello, World!")`

## 基本
### 予約語
```
後で書く
```

### コメント
"//"から行末までをコメントとみなす. \
`// コメント`

### ブロック
ブロックは{}で作成する.

### 入出力
#### 入力
変数に値をキーボードから入力するための関数は以下のようなものがある. \
```
構文
(get [type]　arg)

例
(get [int] (let [int] even))
[キーボード入力として2を[int]として変数evenに束縛]
even = 2
```

#### 出力
```
出力
(print arg)
改行出力込みで出力
(println arg)

```

## 型
[void]型もある

### 変数
変数の値は原則イミュータブルで変更することができない. また,変数に値を"代入"するではなく"束縛"すると考えてほしい.\
宣言方法は以下のように行う. \
```
(let [int] num <- 10)
(let [float] pi <- 3.14159265)
(let [char] c <- 'a')
(let [string] name <- "Hoge huga")
(let [bool] cond <- True)
```
型を明示的に宣言し,変数名にデータをアローを用いて値を束縛する. \

また,letの後にmutを記述することによって変数を可変なものとして扱うことができる.
### 数値
以下のようなの数値を扱うことができる. \
型は`[int],[float]`の2つである
```
123 ...10進数(int)
1.23 ...　浮動小数点(float)
```

### 文字
以下のような文字を扱うことができる. \
型は`[char]`である
```
'a' ... 単一の文字
```

### 文字列
以下のような文字列を扱うことができる. \
型は`[string]`である. \
また、stringとcharのリストは同一のものとして考えてよい. \
```
"Hoge huga" ... 文字列
```

### ブール
以下のようなブール値を扱うことができる. \
型は`[bool]`である. \
```
True ... 真
False ... 偽
```
### エスケープシーケンス
いったん考えない

### リスト
リストは以下のように宣言する. \
```
(let [int] nums <- [1,2,3])
(let [char] chars <- ['a', 'b', 'c'])
```
なお,先ほど既述したように、charのリストとstringは同一のものとして扱われる.
```
(let [char] chars <- ['a', 'b', 'c'])
(let [string] str <- "abc")
```
そのため上に示したcharsとstrは同一である.
### タプル
タプルは以下のように宣言する. \
```
(let [int] nums <- (1,2,3))
(let [char] chars <- ('a', 'b', 'c'))
```

~~これほんとにいるか？~~

## 演算子
以下のような演算子を使うことができる. \
```
算術演算子
+ ...加算
- ...減算
* ...乗算
/ ...除算
% ...剰余

論理演算子
! ...論理否定
|| ...論理和
&& ...論理積

比較演算子
== ...等価
!= ...不等価
< ...未満
<= ...以下
> ...超過
>= ...以上
```

## 関数
### 関数作成
関数は以下のようにして作成することができる. \
```
無名関数の場合
([type var] [type var] -> | process |)

関数の場合
(let [fn] func <- [type var] [type var] -> | process |)

この構文は一見複雑に見えるが,分解してみると下に記述しているように":"で区切った右側と左側に分かれる. 
右側は無名関数の定義.左側は定義した無名関数に名前を付ける操作となっている.
(let [fn] func <- : [type var] [type var] -> | process |)

例
関数定義
(let [fn] add <- [int x] [int rhs,int lhs] -> | (+ rhs lhs) |)
(let [fn] times <- [int x] [int rhs,int lhs] -> | (* rhs lhs) |)

使用
(add 3 7) => 10
(times 3 7) => 21
```

### 関数合成
関数を合成することができる. \
関数合成は以下のように行う. \
```
構文
(synth [fn] func3 <- func1,func2)

例
(let [fn] sqr <- [intx ] [int x] -> | (* x x) | )
(synth [fn] print_sqr <- print,sqr )
(print_sqr 6) => (標準出力として)36
```
関数合成では`(synth [fn] func3 <- func1 func2)`の構文で使用された関数func1,func2のうち処理順はfunc2 -> func1の順番となっている. \
そのため,例で示している関数print_sqrではもととなっている関数print,sqrのうちsqr -> printの順で処理が行われている. \

### 引数,戻り値
関数の引数、戻り値の指定は以下のように行う。
```
(let [fn] add <- [int x]//ここで戻り値の指定 [int rhs,int lhs]//ここで引数のして負い -> | (+ rhs + lhs) |)

(add 5 6) -> 5+6の結果をaddの値として返す　
```

## 制御構文
制御構文はif-else,match,loop,whileがある. \
### if-else

```
(if condition 
 then -> data 
 else -> data)
```
### match
```
(match value {
 cond1 -> data1,
 cond2 -> data2,
 ....
 }
)
```

### loop
`|process|`をtimes回実行する
```
(loop times -> |process|)
```
### while
`|process|`をconditionがTrueの間実行する
```
(while condition -> |process|)
```


## データ型
### struct
### enum

## クラス


# 言語仕様
関数の記述方法 \
(funcname arg) \

(let [type] name <- data) => 変数宣言 \
([type] [type variable] -> | process |) => 無名関数 \
(let [fn] name <- [type] [type var] -> | process |) => 関数作成 \

(let [fn] sqr <- [int] [int x] -> |(* x x)|) \
(sqr 3) => 9 \

例 
```
(let [fn] fizzbuzz <- [string msg] [int num] ->  
  |(match num{  
    num % 15 -> "FizzBuzz", 
    num % 3 -> "Fizz", 
    num % 5 -> "Buzz",} 
  )| 
) 
```



## 実際の使用例
```
(let [fn] main <- [int x] [void] -> 
  (let [int] n <- 0)
  (loop 30 -> (println (fizzbuzz (+ n 1))))
)

(main)
```
上のコードは0～29のfizzbuzzの結果を出力するプログラムである.
