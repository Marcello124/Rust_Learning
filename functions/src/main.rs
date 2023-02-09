fn main() {
    println!("Hello, world!");

    another_function(2, 's');

    let y = {
        let x = 3;
        x + 1
    };

    println!("Value of y is {y}");

    let z = five();

    println!("Value of z is {z}");

    let t = plus_one(5);

    println!("Value of t is {t}");
}

fn another_function(x : i32, unit: char) {
    println!("Value of x is {x}{unit}");
}

fn five() -> i32 {
    4
}

fn plus_one(x: i32) -> i32 {
    x + 1
}