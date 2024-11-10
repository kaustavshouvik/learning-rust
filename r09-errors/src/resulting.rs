mod expecting;
mod return_to_caller;
mod unwrapping;

use std::fs::File;

// Result enum:
// Ok(T) -> Result of a success operation.
// Err(E) -> Result of an error operation.

pub fn resulting() {
    let file = File::open("text.txt");
    dbg!(&file);

    // If error is because of file not found -> create it.
    // Else panic.

    // let file = match file {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("text.txt") {
    //             Ok(file) => file,
    //             Err(_) => panic!("Unable to create file"),
    //         },
    //         _ => panic!("Unable to open file"),
    //     },
    // };

    // dbg!(&file);

    // unwrapping::unwrapping();
    // expecting::expecting();
    return_to_caller::return_to_caller();
}
