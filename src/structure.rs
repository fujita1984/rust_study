use std::fmt;
//VBAのTypeのようなもの
pub fn structure() {
    struct Animal {
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
    }

    let person = Animal {
        animal_type: "human".to_string(),
        name: String::from("fujita"),
        arms: 2,
        legs: 2,
    };

    println!("{} {}", person.animal_type, person.name);
}

pub fn tuple_like() {
    struct Loc(i32, i32);
    let parts1 = Loc(1, 5);
    println!("{} {}", parts1.0, parts1.1,);
}

pub fn enum_1() {
    enum Fruits {
        Orange = 12345,
        Apple = 55555,
    }
    let my_fruits = Fruits::Apple;
    println!("{}", my_fruits as usize);
}
