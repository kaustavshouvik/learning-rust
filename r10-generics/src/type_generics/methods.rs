#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

// Whatever type parameter we want to use inside
// the impl block, we declare them right after 'impl'.
// impl<A, B, C>.

impl<T, Y> Point<T, Y> {
    fn get_x(&self) -> &T {
        return &self.x;
    }
}

// This impl block specifies concrete types for the struct.
// It means that methods defined in this block will only
// be available if it matches those concrete types.
impl Point<char, char> {
    fn print_xy(&self) {
        println!("{0}{1}", self.x, self.y);
    }
}

pub fn methods() {
    let p1 = Point { x: 10, y: 5.5 };
    dbg!(p1.get_x());

    let p2 = Point { x: 'A', y: 'B' };
    p2.print_xy(); // <-- Only available for this point.

    let p3 = Point { x: 'A', y: 0 };
    // p3.print_xy() // <-- Nope.

    dbg!(&p3);
}
