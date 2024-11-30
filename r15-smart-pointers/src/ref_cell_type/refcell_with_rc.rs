use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum ConsList {
    Value(Rc<RefCell<i32>>, Rc<ConsList>),
    Nil,
}

fn mutate_inner(list: &ConsList) {
    match list {
        ConsList::Value(val, _) => {
            *val.borrow_mut() = 20;
        }
        _ => {}
    }
}

pub fn refcell_with_rc() {
    let val = Rc::new(RefCell::new(10));

    let a = ConsList::Value(Rc::clone(&val), Rc::new(ConsList::Nil));

    // 'Rc' and 'RefCell':
    //      Allows to have multiple ownership
    //      and each one can mutate the inner value.

    println!("{a:?}");

    mutate_inner(&a);
    println!("{a:?}");

    *val.borrow_mut() = 15;
    println!("{a:?}");
}
