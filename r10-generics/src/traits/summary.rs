// We define method abstractions here,
// which then we'd implement on concrete types.
pub trait Summary {
    fn summarize(&self) -> String;

    // Can also provide a default implementation.
    // fn summarize(&self) -> String {
    //     String::from("Read more(...)")
    // }
}
