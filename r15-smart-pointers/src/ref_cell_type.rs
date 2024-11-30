mod refcell_with_rc;

use std::cell::RefCell;

trait Sender {
    fn send(&self, msg: &str);
}

struct Value<'a, T: Sender> {
    sender: &'a T,
    val: i32,
}

impl<'a, T> Value<'a, T>
where
    T: Sender,
{
    fn update(&mut self, new_val: i32) {
        self.val = new_val;

        self.sender.send(&format!("Value updated to {0}", self.val));
    }
}

struct Queue {
    // messages: Vec<String>,
    messages: RefCell<Vec<String>>,
}

impl Sender for Queue {
    // Using '&mut self' is invalid and the trait
    // only allows '&self' (immutable reference).

    // So we use 'RefCell', it allows interior mutability.

    // Meaning an instance field is allowed to be mutated
    // from an immutable instance of the type.

    fn send(&self, msg: &str) {
        // self.messages.push(String::from(msg));

        // 'borrow_mut' -> returns a mutable reference.

        self.messages.borrow_mut().push(String::from(msg));
    }
}

impl Queue {
    fn new() -> Queue {
        Queue {
            // messages: vec![]
            messages: RefCell::new(vec![]),
        }
    }
}

pub fn ref_cell_type() {
    let q = Queue::new();
    let mut val = Value { sender: &q, val: 0 };

    val.update(10);
    val.update(20);

    // To see the contents of 'messages' of 'q':
    assert_eq!(q.messages.borrow().len(), 2);
    dbg!(q.messages.borrow());

    // Problem:
    //  Need to update an instance field
    //  but trait signature only allows
    //  mutable reference.

    // And in rust it is not allowed to mutate
    // a value or fields (in case of structs)
    // unless the instance is mutable itself.

    // Solution:
    //  -> Interior mutability.
    //  -> It allows us to create a mutable reference
    //      out of an immutable reference.
    //  -> Which enables mutating an inner value
    //      of an immutable instance.

    // By 'inner' value I mean the value which
    // the variable holds:
    //  -> Primitive: Inner value is same as the value.
    //  -> Structs: The instance fields.

    // Note that, interior mutability only moves
    // the rust borrowing rules checking at runtime
    // instead of compile time.

    // So yes, there is a little bit of cost
    // that'll be incurred at runtime.

    // So, if we were to violate any rule at runtime.
    //  -> It panics.

    // This panics:
    // let mut_ref_1 = q.messages.borrow_mut();
    // let mut_ref_2 = q.messages.borrow_mut();

    // As multiple mutable borrows at a time is not allowed.

    refcell_with_rc::refcell_with_rc();
}
