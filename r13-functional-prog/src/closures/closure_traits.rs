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

    // The 'FnMut' traits says the the closure
    // you're passing to me is allowed to mutate
    // captured values but is not allowed to
    // take ownership of the captured value.

    // The closure that is passed here however,
    // does not even capture anything to begin with.
    //  -> It implements the 'Fn' trait.
    v.sort_by_key(|r| r.w);

    // Yes, there is a hierarchy going on:
    //  -> Whatever accepts 'Fn' accepts 'FnMut' & 'FnOnce' too.
    //  -> Whatever accepts 'FnMut' accepts 'FnOnce' too.
    //  -> Whatever accepts 'FnOnce' accepts only that.

    // Let's consider another closure:

    // let s = String::from("SORTING FUNCTION CALLED");
    // let mut calls: Vec<String> = Vec::new();
    // let mut fOnce = |r: Rect| {
    //     calls.push(s);
    //     r.w
    // };

    // The 'fOnce' closure implements 'FnOnce' trait
    // and can't be passed to 'sort_by_key' because
    // of the 2nd point about the hierarchy.

    // Closures typed as 'FnOnce', can only be called
    // once because they captured a value by taking ownership
    // and "moves out the owned value from its scope".

    // In this case, since 'calls' take a owned strings,
    // it moves "s" out of the outer scope where it was created.

    println!("After sort: v = {v:?}");
}
