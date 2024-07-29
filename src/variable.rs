//覚えておいた方がいい英語
//mutable=>可変
//immutable=>不変

//変数と定数の記述方法
//変数=>max_value　小文字のスネークケース
//定数=>MAX_VALUE　大文字のスネークケース

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

//数値を計算する場合は同じ型にしなければいけない
//異なる型で計算しようとするとエラーになる
//error[E0308]: mismatched types

//mainで呼び出すためpubとする
pub fn var_num() {
    const CONST_VALUE: i32 = 10;
    let immutable_var: i32 = 20;
    let mut mutable_var: i16 = 30;

    //mutableなので変更できる
    mutable_var = 15;

    //異なる型で計算をする場合は　[as 型名] で変換する
    let total: i32 = CONST_VALUE + immutable_var + mutable_var as i32;
    println!("{}", total);
}
