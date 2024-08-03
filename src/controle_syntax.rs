//制御構文

pub fn loop1() {
    //loop
    //無限ループ
    //ループ内で条件を指定してbreakする
    loop {
        //処理
        break;
    }

    //while
    //条件がtrueの時繰り返す
    let mut i: i32 = 0;
    while i < 10 {
        //処理
        i += 1;
    }

    //for
    //0から4まで繰り返す
    for x in 0..5 {
        //処理
    }

    //0から5まで繰り返す
    for x in 0..=5 {
        //処理
    }

    //配列の繰り返し
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    for x in array1 {
        print!("{}", x);
    }
}
