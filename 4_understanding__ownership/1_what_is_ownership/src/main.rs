// main function itself is a scope, 
// so when program is done
// all values are dropped (from stack)
fn main() {
    
    // s is valid here
    // s is a string slice (?)
    let _s = "hello";

    // s as a String type stored at heap and with unknown size
    // can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{s}");

    // both x and y have fixed size (i32) and are pushed onto the stack
    let mut x = 5;
    let y = x;

    // when x is incremented y is not
    // y is a copy not a reference
    x += 1;

    println!("{x} {y}");

    // s1 is owner of String "hello"
    let s1 = String::from("hello");
    // s2 becomes owner of String "hello"
    // s1 looses ownership (s1 is moved into s2)
    let s2 = s1;

    // so printing it won't compile
    println!("{s2}");

    let s1 = s2;
    // this is a deep copy of s1
    let s2 = s1.clone();

    println!("{s1} {s2}");

    takes_ownership(s1);
    // s1 went out of scope in this function
    // printing this won't compile
    // println!("{s1}");

    takes_ownership(s2.clone());
    // s2 is copied so it doesn't go out of scope
    // this compiles 
    println!("{s2}");

    makes_copy(x);
    // this was just a copy
    // x doesn't go out of scope
    println!("{x}");

    let s3 = gives_back(s2);
    // s2 went out of scope, but its value was moved to s3
    // println!("{s2}");
    println!("{s3}");
}
// s is not valid here

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_back(some_string: String) -> String {
    println!("{some_string}");
    some_string
}