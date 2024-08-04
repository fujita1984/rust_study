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

pub fn if_match() {
    //if文
    let x: i32 = 5;
    if x > 10 {
        //処理
    } else if x == 5 {
        //処理
    } else {
        //処理
    }

    //match
    //別言語のswitchやcaseのようなもの
    let i: i32 = 7;
    match i {
        0 => {
            println!("zero");
        }

        1 | 2 | 3 => {
            println!("1 or 2 or 3");
        }

        4..=7 => {
            println!("4 to 7");
        }

        _ => {
            println!("else");
        }
    }
}

pub fn let_if_match() {
    let y: i32 = 3;
    let x: i32 = if y < 10 { 5 } else { 2 };
    println!("{}", x);

    let food: &str = "ばななな";
    let my_food: &str = match food {
        "ばなな" => "バナナ",
        _ => "ミカン",
    };
    println!("{}", my_food);
}
