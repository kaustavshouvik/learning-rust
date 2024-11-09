// 'mod' is used to declare a module.
// -> That this code uses a module called 'back_of_house'.
// -> The compiler then looks for the file 'back_of_house.rs',
//      for the code of the module, relative to current directory.
// -> We only specify 'mod <module-name>' once and that too
//      in the root of the crate.
// -> We do similarly for sub-module, where we specify 'mod'
//      in the parent module file and same for sub-sub-.. modules.

mod back_of_house;
mod front_of_house;

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
