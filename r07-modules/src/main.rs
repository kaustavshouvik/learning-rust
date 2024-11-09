// Module -> Defined inline.
mod front_of_house {
    // Sub-module.
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

mod back_of_house {

    // Marking the struct public only allows
    // ancestor modules to refer to it and
    // call it's public methods.
    //
    // Each field that needs to public
    // needs to be marked explicitly.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // On contrary to structs, marking an enum public
    // makes all of its variants public as well.
    #[derive(Debug)]
    pub enum Drink {
        Juice,
        Soda,
        Lemonade,
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Self {
            Breakfast {
                toast,
                seasonal_fruit: String::from("Guava"),
            }
        }
    }

    // Marking them pub -> as it is being used
    // in an ancestor module.
    //
    // Marking them pub has nothing to do with
    // using stuff that is defined in ancestor module.
    pub mod orders {
        pub fn process_order() {
            // 'super' is like '..' when navigating fs in os.
            super::super::deliver_order();
        }
    }
}

// Notice that we didn't need to declare 'font_of_house'
// as public because the code which is making use of it,
// is at the same level.

// By 'making use of it' we mean refer to it.
// It still can't access stuff inside it.

// We only mark a module (or things inside) as public,
// if an ancestor module wants to use it.

// An inner module will have access to ancestor modules
// and it's contents.

// To create shortcut to a thing defined in module.
use front_of_house::hosting;

mod customer {
    // Notice that use only works in the scope it is used.
    // Since module is a different scope,
    // the parent 'use' doesn't work here.

    // To use hosting directly, we use 'use' here too.
    use super::front_of_house::hosting;

    pub fn enter_restaurant() {
        hosting::add_to_waitlist();
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
    back_of_house::orders::process_order();

    // Structs

    let meal = back_of_house::Breakfast::summer(String::from("French"));

    println!("meal.toast = {0}", meal.toast);

    let soda = back_of_house::Drink::Soda;
    dbg!(soda);

    // Since we used 'use' to create shortcut to 'hosting'.
    // We can directly use things inside 'hosting'.
    hosting::add_to_waitlist();

    customer::enter_restaurant();
}

fn deliver_order() {
    println!("Delivering order");
}
