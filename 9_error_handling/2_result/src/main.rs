use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");
    // println!("{:#?}", greeting_file_result);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_create) => file_create,
                Err(error) => panic!("Problem creating file: {:?}", error),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

    // This does exactly the same as the above
    // For now this is less readable for me
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt is missing");

    // main returns (), File::open returns File
    // They are incompatible so it won't compile
    // let _greeting_file = File::open("hello.txt")?;

    // let username_result = read_username_from_file("hello.txt").unwrap();
    // println!("{}", username_result);

    Ok(())
}

// fn read_username_from_file(file: &str) -> Result<String, io::Error> {
//     let username_file_result = File::open(file);

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// A lot shorter implementation
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

// The shortest (and the best) way
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}