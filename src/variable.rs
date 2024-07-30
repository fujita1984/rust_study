//覚えておいた方がいい英語
//mutable=>可変
//immutable=>不変

//変数と定数の記述方法
//変数=>max_value　小文字のスネークケース
//定数=>MAX_VALUE　大文字のスネークケース

//領域
//変数自体や数値型=>スタック
//文字列やベクタ等=>ヒープ
//定数や&str=>静的

//変数はデフォルトで不変(immutable)なのでこのコードはコンパイルエラーになる
// let x: i32 = 10;
// x = 20;
// println!("{}", x);
//ERROR[E0384]: cannot assign twice to immutable variable `x`

//変数を可変にするには変数名の手前に mut と書く(mut => mutableの意味)

//定数と変数の違い
//型：定数=>指定必須　変数=>推論可
//評価：定数=>コンパイル時に評価  変数=>実行時に評価
//スコープ：定数=>グローバルに定義できる(モジュール定義推奨) 変数=>ブロック内

//mainで呼び出すためpubとする
pub fn var_num() {
    const CONST_VALUE: i32 = 10;
    let immutable_var: i32 = 20;
    let mut mutable_var: i16 = 30;

    //mutableなので変更できる
    mutable_var = 15;

    //数値を計算する場合は同じ型にしなければいけない
    //異なる型で計算しようとするとエラーになる
    //error[E0308]: mismatched types
    //異なる型で計算をする場合は　[as 型名] で変換する
    let total: i32 = CONST_VALUE + immutable_var + mutable_var as i32;
    println!("{}", total);
}

//Stringと&str
//String=>可変 &str=>不変
//String=>所有 &str=>参照
//String=>柔軟 &str=>軽量

pub fn var_string() {
    //どちらでも文字列を定義できる。違いはまだわからない。
    let string1: String = String::from("Hello!");
    // let string1: String = "String1".to_string();

    //cloneは値渡しなので別のメモリに同じ文字列がコピーされる
    //値渡しなので所有権の移動は起こらない
    let string2: String = string1.clone();

    //=変数名と書くと所有権が移動してstring1は使えなくなる
    let mut string3: String = string1;

    //他の言語なら普通の書き方だがrustではエラーになる
    //tostringやfromを使わない文字列は&str型と判断されStringと型違いになる
    //string3 = "abcde";

    //エラーになりそうでならない
    //rustでは再定義が認められている　シャドーイングと言うらしい
    let string3: String = "World!".to_string();

    //+で文字列を結合する場合、1番目はString型で2番目以降は&str型(&をつける)
    //+で文字列を結合した場合、1番目のString型は所有権が移動する
    // println!("{}", string2 + &string3);

    //formatマクロを利用した場合は所有権は移動しない
    println!("{}", format!("{}{}", string2, string3));
}
