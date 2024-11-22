// Closures remembers values of outer world,
// by taking borrows or ownership, just like functions.

use std::thread;

fn immutable_borrow() {
    let v = vec![1, 2, 3];

    let f = || println!("In closure: v = {v:?}");

    // Immutable borrow.
    println!("Before calling closure: v = {v:?}");
    // Immutable borrow.
    f();
    // Immutable borrow.
    println!("After calling closure: v = {v:?}");
}

fn mutable_borrow() {
    let mut v = vec![1, 2, 3];

    // Note that this variable is marked as mutable.
    let mut f = || {
        v.push(4);
        println!("In closure: v = {v:?}");
    };

    // When you're modifying a value inside closure,
    // (or even outside closure) that value must be mutable.

    // Since a closure need to modify the value,
    // it has to take the value as a mutable borrow
    // (just like a function).

    // And a closure taking a mutable borrow must be
    // marked as mutable.

    // No print lines here, as other borrows of any kind
    // are not allowed when using a mutable borrow in later code.
    f();
}

fn taking_ownership() {
    let v = vec![1, 2, 3];

    // We take make a closure take ownership of a value
    // even if it doesn't necessarily need the ownership.

    // It is done using the 'move' keyword.
    thread::spawn(move || println!("In thread: v = {v:?}"))
        .join()
        .unwrap();

    // We need to 'move' the value to the spawned thread
    // because the main thread might finish before the thread
    // and in that case the value will be dropped and the thread
    // will have an invalid reference.
}

pub fn capturing() {
    // immutable_borrow();
    // mutable_borrow();
    taking_ownership();
}
