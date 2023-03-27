mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }    
}

mod back_of_the_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

pub fn eat_at_restaurant() {
    // This is an absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // This is a relative path
    front_of_house::hosting::add_to_waitlist();

    // Only toast can be changed, becouse it's public. 
    // fruit is private
    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn deliver_order() {}
