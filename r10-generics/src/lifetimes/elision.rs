// Elision -> A set of rules, the will compiler follow
//      these rules to determine the lifetimes
//      of references on its own.

use super::structs::IHaveARef;

// One of the rule says, if it's a method
// and the return value is a ref, that lifetime
// of the return value is the same as
// the lifetime of the instance of type.

// Since it accepts a lifetime generic parameter
// this is not valid, we must annotate.
// impl IHaveARef {
impl<'a> IHaveARef<'a> {
    fn get_ref_value(&self, _a: &i32) -> &str {
        self.refValue
    }
}

pub fn elision() {
    let x = IHaveARef { refValue: "ABCD" };

    let a = 10;
    let ref_val = x.get_ref_value(&a);

    dbg!(ref_val);
}
