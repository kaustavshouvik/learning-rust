#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

pub fn structs() {
    let p1 = Point { x: 2, y: 3.5 };
    let p2 = Point { x: 2.0, y: 4.5 };

    dbg!(&p1);
    dbg!(&p2);
}
