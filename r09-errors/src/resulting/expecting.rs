use std::fs::File;

pub fn expecting() {
    // A better alternative to 'unwrap()' is to call 'expect(msg)'
    // in the result enum, it takes a custom message as an argument.

    let file = File::open("hello.txt").expect("no file -> hello.txt");

    // This would return Ok() value if present.
    // In case of Err(), it panics with:
    // -> First displays the custom message.
    // -> Then debug prints the Err() variant.

    dbg!(&file);
}
