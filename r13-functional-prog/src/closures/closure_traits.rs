// Closures implements a specific set of traits.
// These traits governs the usage of closures.

// Read the docs -> later half of 13.1.

pub fn closure_traits() {
    #[derive(Debug)]
    struct Rect {
        w: i32,
        h: i32,
    }

    let mut v = vec![
        Rect { w: 10, h: 10 },
        Rect { w: 5, h: 5 },
        Rect { w: 15, h: 15 },
    ];

    println!("Before sort: v = {v:?}");

    // This 'sort_by_key' method takes a closure
    // which implements 'FnMut' trait.

    // The 'FnMut' is one of the specific closure traits.

    // This traits says the the closure is allowed to
    // mutate captured values but is not allowed to
    // move values out of the closure.

    v.sort_by_key(|r| r.w);

    // Let's consider another closure:

    // let s = String::from("SORTING FUNCTION CALLED");
    // let mut calls: Vec<String> = Vec::new();
    // let mut fOnce = |r: Rect| {
    //     calls.push(s);
    //     r.w
    // };

    // Can not use with 'sort_by_key' because 'fOnce'
    // implements 'FnOnce' trait.

    // Closures typed as 'FnOnce', can only be called
    // once because they captured a value by taking ownership
    // and moves out the owned value, when a owned value
    // is moved out once calling it again is invalid.

    println!("After sort: v = {v:?}");
}
