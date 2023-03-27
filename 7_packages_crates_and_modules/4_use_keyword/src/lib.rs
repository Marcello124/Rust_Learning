mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// when using functions - specify parent path
use crate::front_of_house::hosting;
//when using structs or enum - specify whole path
use std::collections::hash_map;

mod customer {
    pub fn eat_at_restaurant() {
        // use is out of scope
        // you need to use super::
        super::hosting::add_to_waitlist();
    }
}
