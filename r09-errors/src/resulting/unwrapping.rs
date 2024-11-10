use std::fs::File;

pub fn unwrapping() {
    // Matching is a little to verbose.

    // If we a simple use case and it's fine
    // to exit the process if the file can't
    // be opened, we might want to use 'unwrap()'.

    let file = File::open("text.txt").unwrap();

    // Unwrap is equivalent to:
    // let file = match file {
    //     Ok(file) => file,
    //     Err(err) => panic!("Unable to open file: {err:?}"),
    // };

    // If result is OK():
    // -> It returns the value in Ok() variant.
    // If result is Err():
    // -> It panics.

    dbg!(&file);
}
