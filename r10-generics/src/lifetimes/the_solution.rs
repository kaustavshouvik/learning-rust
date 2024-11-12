// More like 'a solution'.

// To help compiler determine the lifetime of the return value
// from this function, we add lifetime annotations,
// the weird "'a" syntax, it is a generic parameter.

// By annotating the same value to parameters
// and the return value, it says that the return
// value is valid as long as both input parameters
// are valid.

// -> Lifetime of the return value is
//      min(lifetime of a, lifetime of b).

// It is like a rule which we tell to the compiler
// and if this rule is ever violated, the program
// would not compile.

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }

    return b;
}

fn works() {
    let a = String::from("abcd");
    let b = String::from("xyz");

    let res = longest(&a, &b);

    dbg!(res);
}

fn works_varying_scope() {
    let a = String::from("ab");

    {
        let b = String::from("xyz");
        let res = longest(&a, &b);

        dbg!(res);
    }
}

// fn does_not_work() {
//     let a = String::from("ab");
//     let res;
//
//     {
//         let b = String::from("xyz");
//         res = longest(&a, &b);
//     }
//
//      This example doesn't work because,
//      'res' is used after 'b' is dropped.
//
//      This violates lifetime constraint
//      which we defined, so it doesn't compile.
//
//     dbg!(res);
// }

pub fn the_solution() {
    works();
    works_varying_scope();
    // does_not_work();
}
