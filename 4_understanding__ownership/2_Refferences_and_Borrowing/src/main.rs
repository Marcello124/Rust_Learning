fn main() {
    // for String to be changed it has to be mutable
    let mut s1 = String::from("hello");

    // We also have to pass a mutable reference 
    change(&mut s1);
    {
        let r1 = &mut s1;
    } // r1 goes out of scope so it's dropped
    // let r2 = &mut s1; // If there is mutable reference, there cannot be more references
    let r3 = &s1;
    let r4 = &s1; // There can be multiple immutable references


    println!("{} {}", r3, r4);

    let len = calculate_length(&s1); // &s1 is a reference to s1

    println!("The length of {s1} is {len}");
}

// This function takes in a reference to a String 
// it's like a pointer, but poitner can point into no longer used address
// which is Undefined Behavior (UB)
fn calculate_length(s: &String) -> usize {
    // we no longer have to return String and len in a tuple
    // becouse String is borrowed, it isn't owned
    s.len() 
}

// To change string we have to pass a &mut reference
// becouse references are immutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s // this is just a normal value
    // &s // this is a reference to s
} // but s goes here out of scope, so there is a dandling reference
  // rustc forbids dandling references
