mod recursive_types;

fn boxing_basics() {
    // This enables us to store '10' in heap.
    let x = Box::new(10);
    println!("Boxed x = {x}");

    // 'x' (the box) goes out of scope at
    // the end of 'boxing_basics' and is dropped.
    // And with that, the value (on the heap)
    // is dropped as well.
}

pub fn boxing() {
    // boxing_basics();
    recursive_types::recursive_types();
}
