mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// when using functions - specify parent path
// use crate::front_of_house::hosting;

//when using structs or enum - specify whole path
use std::collections::HashMap;

// if you need to destinguish 2 enumms or structs
// import with parent module
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// you can also use "as"
use std::io::Result as IoResult;

// re-exporting - shortens path when using in external library
pub use crate::front_of_house::hosting;
// restaurant::hosting::add_to_waitlist;
// instead of
// restaurant::front_of_house::hosting::add_to_waitlist;

// nested paths with commont parts
use std::cmp::{Ordering, PartialEq};

// you can also use self
use std::io::{self, Write};

// glob operator (be careful!)
use std::collections::*;

mod customer {
    pub fn eat_at_restaurant() {
        // use is out of scope
        // you need to use super::
        super::hosting::add_to_waitlist();
    }
}
