use std::{collections::HashMap, hash::Hash, io::Stdin};

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team_name = String::from("Blue");

    // this is a lot
    // get gets Option<&V>, which is unwrapped and copied if there is a value 
    // or defaults to 0 if there is no Value for that Key 
    // Option<&V> -> Option<V> -> V
    //     copied ^^    unwrap ^^
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // map took ownership of these Strings
    // println!("{}", field_name);

    // Overwrite
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("\nOverwrite\n{:?}", scores);

    // Adding a key and value if key isn't present
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    println!("{:?}", scores.entry(String::from("Red")));
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(25);

    println!("\nAdd\n{:?}", scores);

    // Update
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
