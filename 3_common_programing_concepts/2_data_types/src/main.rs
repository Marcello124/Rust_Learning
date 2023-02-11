fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope is {x}");
    }

    println!("The value of x in outer scope is {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    let sum = 5 + 10;
    let difference = 92.1 - 22.4;
    let product = 4 * 44;
    let quotient = 53.2 / 22.3;
    let truncated = 5 / 2; // it is integer, becouse it is dividing int by int
    let remainder = 43 % 5;

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        sum, difference, product, quotient, truncated, remainder
    );

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';
    // let smt: char = 'U+9FFF';

    let tup: (i32, f64, u8) = (2, 2.8, 1);
    let (x, y, z) = tup;
    println!("{tup:?}\n{x} {y} {z}");
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr: [i32; 5]= [1, 2, 3, 4, 5];
    let six = [6; 6];
    println!("{arr:?}\n{six:?}\n{}", arr[1]);

    println!("Enter index number");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index is not a number");

    let element = arr[index];
    println!("Value of element of an array at index {index} is {element}")
}
