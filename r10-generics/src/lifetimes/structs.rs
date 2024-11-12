#[derive(Debug)]
// The value pointing to the ref
// that would be used to create
// an instance have live longer
// than the instance.
pub struct IHaveARef<'a> {
    pub refValue: &'a str,
}

fn works() {
    let val = String::from("Wattson");
    let x = IHaveARef { refValue: &val };

    dbg!(&x); // val (the value), x (the instance)
              // both live this here, so it works.
}

// fn not_works() {
//     let x;
//
//     {
//         let val = String::from("Wattson");
//         x = IHaveARef { refValue: &val };
//     } // val lives till here.
//
//     dbg!(&x); // instance lives till here
//               // instance lives longer -> WRONG.
// }

pub fn structs() {
    works();
}
