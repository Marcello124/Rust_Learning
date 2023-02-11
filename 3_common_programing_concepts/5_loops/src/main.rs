fn main() {
    let mut count = 0;

    'counting_loop: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("End of loop!");

    println!("Listing all elements of the list:");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Value at index {index}: {}", a[index]);

        index += 1;
    }


    println!(r#"Alternative way using "for loop""#);

    for element in a {
        println!("Value is {element}")
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
