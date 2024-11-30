// We use Rc<T> when multiple parts needs access
// to some data in heap and we don't know which
// will finish using it last, at compile time.

// Rc<T> enables a value to have multiple owners.

// If it were known at compile time, we could have
// used references or transfer ownership.

use std::rc::Rc;

#[derive(Debug)]
enum ConsList {
    Value(i32, Rc<ConsList>),
    Nil,
}

pub fn rc_type() {
    let a = Rc::new(ConsList::Value(
        5,
        Rc::new(ConsList::Value(7, Rc::new(ConsList::Nil))),
    ));

    // -> (5, (7, Nil))

    // 'Rc::clone' increases reference count of 'a'.

    let b = ConsList::Value(2, Rc::clone(&a)); // +1.
    let c = ConsList::Value(3, Rc::clone(&a)); // +1.

    // b -> a
    //      ^
    // c ___|

    // 'Rc::clone' only increments the reference count.
    // It doesn't do deep copy.

    dbg!(&b);
    dbg!(&c);

    // Why not 'Box'?
    //  Assume the 2nd item in 'Value' is 'Box' type:
    //
    //  let a = ConsList::Value(10, Box::new(ConsList::Nil));
    //  let b = ConsList::Value(20, Box::new(a));
    //      --> Here 'a' is moved out.
    //  let c = ConsList::Value(30, Box::new(a));
    //      --> Here 'a' is not usable anymore.
}
