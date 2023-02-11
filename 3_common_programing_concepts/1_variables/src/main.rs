fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    let _spaces = "   ";
    let _spaces = _spaces.len();
}