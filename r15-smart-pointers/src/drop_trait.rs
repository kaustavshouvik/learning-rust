// 'Drop' is included in prelude.

#[derive(Debug)]
struct MyType {
    val: i32,
}

impl Drop for MyType {
    // Runs when instance of 'MyType' goes out of scope.
    fn drop(&mut self) {
        println!("Dropping {self:?}, bye...");
    }
}

pub fn drop_trait() {
    let x = MyType { val: 10 };
    let y = MyType { val: 20 };

    println!("Instantiated x ({x:?}) and y ({y:?})");

    // 'x' and 'y' both goes out of scope
    // at the end of main -> drop is called.

    // When it's required to call drop at a point
    // before it goes out of scope, we use 'std::mem::drop'.
    // Which is also included in the prelude.

    drop(x); // <-- Takes ownership.

    println!("Y still exists -> {y:?}");
}
