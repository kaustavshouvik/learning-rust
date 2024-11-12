// Borrow checker is the part of compiler
// that ensures the a reference is valid.

// Without lifetimes -> DOESN'T COMPILE.
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         return a;
//     }
//
//     return b;
// }

pub fn the_problem() {
    let a = String::from("abcd");
    let b = String::from("xyz");

    // let res = longest(&a, &b);

    // Here we're passing ref to 'a' and ref to 'b'
    // to a function which returns a ref.

    // But the compiler doesn't know which ref is
    // actually returned, if the source strings
    // were coming from input we don't know either.

    // Compiler be like:
    //      Does it return back the ref to 'a'
    //      or the ref to 'b'? I've no idea.

    // Note that, if a function returns a ref,
    // it must be one of the ref that was passed
    // to the function, since rust doesn't allow
    // returning a ref to a thing that is created
    // inside the said function.

    // The compiler knows the lifetimes of 'a' and 'b',
    // but doesn't know the lifetime of 'res'.

    // Then it can not determine which lifetime is longer.
    // Note that a lifetime of a value must be longer than
    // the lifetime of a ref, only then the program compiles.
}
