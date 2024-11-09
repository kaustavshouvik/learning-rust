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
