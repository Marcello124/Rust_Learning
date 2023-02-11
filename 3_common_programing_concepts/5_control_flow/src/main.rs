fn main() {
    let number = 7;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Numebr is divisible by 2");
    } else {
        println!("Numebr is not divisible by 4, 3 nor 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number has a value: {number}");
}