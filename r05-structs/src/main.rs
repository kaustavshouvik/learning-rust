struct User {
    name: String,
    email: String,
    sign_in_count: u32,
}

// 'derive()' is an attribute.
// 'Debug' is a trait.
// This implements the Debug trait.
// Which allows us to debug print a struct.

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // '&self' is short for 'self: &Self'.
    // '&Self' is an alias for type of the impl block.
    // Essentially, it becomes 'self: &Rectangle'.

    fn area(&self) -> u32 {
        self.height * self.width
    }

    // Every function defined here is an associated function.
    // We can have associated function without 'self' parameter.
    // These are typically used to construct new instance of the type.

    fn square(size: u32) -> Self {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    // struct_basics();
    // struct_from_struct();
    // printing_structs();
    methods();
}

fn struct_basics() {
    let a = User {
        name: String::from("Octane"),
        email: String::from("oct@apex.com"),
        sign_in_count: 1,
    };

    // Accessing fields.
    let a_name = a.name;
    println!("a's name = {a_name}");

    // Can not update field.
    // a.email = String::from("octy@apex.com")

    // For fields to be updatable,
    // declare a mutable struct.

    let mut b = User {
        name: String::from("Horizon"),
        email: String::from("horizon@apex.com"),
        sign_in_count: 2,
    };

    // A struct can not have selective mutable fields.
    // For even a single field to be mutable,
    // the entire struct has to be mutable.

    b.email = String::from("y-axis@apex.com");

    println!("b.name = {0}", b.name);
    println!("b.email = {0}", b.email);
}

fn struct_from_struct() {
    let a = User {
        name: String::from("Octane"),
        email: String::from("oct@apex.com"),
        sign_in_count: 1,
    };

    // To create a struct with only few changed fields.
    let b = User {
        sign_in_count: 2,
        ..a
    };

    // This "moves" heap fields.
    // -> a.name is no longer valid.

    // println!("a.name = {0}", a.name);
    println!("b.name = {0}", b.name);

    // If you only keep the remaining fields,
    // which are of simple types, those fields
    // will be valid on both structs.
    let x = User {
        name: String::from("Mirage"),
        email: String::from("dup@apex.com"),
        sign_in_count: 1,
    };

    let y = User {
        name: String::from("Wattson"),
        email: String::from("fencing@apex.com"),
        ..x
    };

    // All valid.
    println!("x.name = {0}", x.name);
    println!("x.sign_in_count = {0}", x.sign_in_count);
    println!("y.name = {0}", y.name);
    println!("y.sign_in_count = {0}", y.sign_in_count);
}

fn printing_structs() {
    let a = Rectangle {
        height: 20,
        width: 30,
    };

    println!("a = {a:?}");

    // dbg!():
    // -> Takes and returns ownership.
    // -> Prints to stderr.
    // -> Prints file and line number as well.
    // dbg!(a);

    // Don't wanna transfer ownership.
    dbg!(&a);
}

fn methods() {
    let rect1 = Rectangle {
        height: 25,
        width: 40,
    };

    println!("rect1.area = {0}", rect1.area());

    // It's not really a method.
    let sq = Rectangle::square(10);
    println!("sq.area = {0}", sq.area());
}
