// This doesn't compile:
//
// enum ConsList {
//     Value(i32, ConsList),
//     Nil,
// }
//
// Enums are stored in stack.
//  -> Size must be known at compile time.
// In case of recursive types:
//  -> Size can not be known at compile time.
//
// Note that, by 'size' rust needs to know the size
// of the struct and not the size based on its usage.
//
// Fun fact: Size of an enum is the size
//      of the largest variant.
//

// This one compiles tho:
#[derive(Debug)]
enum ConsList {
    Value(i32, Box<ConsList>),
    Nil,
}

// Since 'Box' is like a container,
// which stores the pointer value
// and the actual data is stored in heap.
//  -> Box size fixed and is known at compile time.
//  -> Which means, size of the enum is known
//      at compile time.

pub fn recursive_types() {
    let x = ConsList::Value(10, Box::new(ConsList::Value(20, Box::new(ConsList::Nil))));
    // -> (10, (20, Nil)).

    println!("{x:?}");
}
