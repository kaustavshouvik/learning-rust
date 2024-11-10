use std::cmp::PartialOrd;

// The 'T: Something' specifies that T can only
// be the types which implements the 'Something' trait.

fn max<T: PartialOrd>(list: &[T]) -> &T {
    let mut res = &list[0];

    for item in list {
        if item > res {
            res = item;
        }
    }

    res
}

pub fn functions() {
    let a1: [i32; 5] = [5, 10, 8, 15, 9];
    let a2 = ['a', 'b', 'd', 'c'];

    let x1 = max(&a1);
    let x2 = max(&a2);

    dbg!(&x1);
    dbg!(&x2);
}
