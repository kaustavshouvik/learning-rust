fn main() {
    // ownership();
    // references();
    slices();
}

fn ownership() {
    // -> String literal.
    // -> Hardcoded in the executable.
    let s1 = "Octane";

    println!("s1 = {s1}");

    // -> String type.
    // -> Stored on the heap.
    let s2 = String::from("Pathfinder");

    println!("s2 = {s2}");

    let a = 10;
    let b = a;

    // For simple types:
    // -> Value of 'a' is copied to 'b'.
    // -> 'b' is an individual value.

    println!("a = {a}, b = {b}");

    let x = String::from("Fuse");
    let y = x;

    // For type which store their data on heap:
    // -> Value of 'x' "moved" to 'y'.
    // -> 'x' is no longer valid after assignment to 'y'.

    // print!("x = {x}, y = {y}"); // Can't use 'x' anymore.
    println!("y = {y}");

    // When passing String type to function:
    // -> The function "takes ownership".
    // -> Passed variable no longer valid after the function call.

    // When returning String type from function:
    // -> It caller function "takes ownership".

    // Want the variable to be usable even after passing to a function?
    // -> REFERENCES.
}

fn references() {
    let s = String::from("Conduit");
    print_len(&s);

    println!("s = {s}"); // Valid here too.

    ref_mutable_ness();
}

fn print_len(s: &String) {
    let len = s.len();
    println!("Length of'{s}' is {len}")
}

fn ref_mutable_ness() {
    let mut s = String::from("Mirage");

    // Borrow -> The act of creating reference.

    let s1 = &s; // Immutable borrow.
    let s2 = &s; // Multiple immutable borrow is fine.

    // Can not modify using immutable borrow.
    // s1.push_str(" is a support legend.");

    // Can not have mutable borrow if there is
    // already an existing immutable borrow.
    // let s3 = &mut s;

    println!("s1 = {s1}, s2 = {s2}");

    // This is fine, because all immutable borrows
    // are 'used' before this line.
    let s4 = &mut s;
    s4.push_str(" is a support legend");

    println!("s4 = {s4}");
}

fn slices() {
    // String slices ->
    let s = String::from("Lifeline");

    // Immutable borrow.
    let s1 = &s[0..4];

    println!("s = {s}, s1 = {s1}");

    // String literals are string slices.
    // String slices point to the memory location of original string.
    // In case of literals since it's hardcoded into the executable,
    // it points to the portion that executable.

    let a = "Catalyst";

    // A function accepting '&str' also accepts '&String'.
    // It's better to write function taking '&str'.

    // Passing '&str' to function accepting '&String',
    // sucks.
    take_string_ref(&String::from(a));

    // Passing '&String' to '&str'.
    take_string_ref(&s);

    // Passing '&str' to '&str'.
    take_string_slice(a);

    // Passing '&String' to '&str'.
    take_string_slice(&s);
}

fn take_string_ref(s: &String) {}

fn take_string_slice(s: &str) {}
