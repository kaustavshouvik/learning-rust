// Smart pointers implements 'Deref' and 'Drop' traits.

// The 'Deref' trait:
//  -> Enables customizing the behavior of '*'
//      (the dereference operator).
//  -> Types implementing 'Deref' traits can be treated
//       as references.
//      -> They can be passed to a function
//          accepting references.

pub fn deref_trait() {
    let x = 10;
    let y = &x;

    // Since smart pointer implements 'Deref' trait.
    let z = Box::new(x);

    assert_eq!(10, *y);

    // Note that, 'z' is not exactly a reference.
    // 'Box' says that whenever someone uses '*'
    // on my instance, I'll return a reference
    // to the boxed value, which rust can dereference.
    assert_eq!(10, *z);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }

    // Something of a custom implementation of 'Box'.
    // Note that this stores data on stack.
    // But we're learning about 'Deref'.
    let my_x = MyBox::new(10);

    // When doing '*my_x' it doesn't work.
    // Rust doesn't know how to dereference it.

    // -----
    // Note that doing '*my_x' will work here
    // because we implemented 'Deref' later
    // (because of hoisting?).
    // -----

    // Implementing the 'Deref' trait:

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // Now it works:
    assert_eq!(10, *my_x);

    // Deref coercions allows us to pass a reference
    // of a type to a function accepting a reference
    // of a different type.

    fn greet(name: &str) {
        println!("Hello, {name}");
    }

    let name = MyBox::new(String::from("Holt"));

    // Greet accepts -> '&str'.
    // We passed -> '&MyBox<String>'.

    // Deref coercion does a series of operations
    // to convert to the required type in the definition.

    // 1. deref(MyBox<String>) -> &String.
    // 2. deref(String) -> &str.
    greet(&name);

    // If rust didn't do this, we would've needed to do:
    // greet(&(*name)[..]);
}
