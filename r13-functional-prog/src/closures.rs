mod capturing;
mod closure_traits;

struct Container {
    value: i32,
}

impl Container {
    // Accepts an 'Option<i32>' as an argument,
    // if the option contains a value, then it
    // it returned or value in the 'Container'
    // instance is returned.
    fn get_value_or_provided(&self, provided: Option<i32>) -> i32 {
        // -> It remembers the things present when it is defined.
        //      It captures an immutable reference of the instance.
        // -> 'unwrap_or_else' on 'Option<T>', the closure must return
        //      the same type 'T'.
        provided.unwrap_or_else(|| self.value)
    }
}

pub fn closures() {
    let container = Container { value: 10 };
    println!("Value = {}", container.get_value_or_provided(None)); // 10
    println!("Value = {}", container.get_value_or_provided(Some(20))); // 20

    // Closures can be defined without any type annotations.
    // Types will be inferred from it's usage and locked.

    // Like a function which accepts 'x' and returns 'x'.
    let from_value = |x| x;

    // Because we used it with 'i32', the closure 'from_value'
    // is now locked to only accept i32s and return that.
    let x = from_value(10);

    println!("x = {x}");

    // Following lines produces error:
    // let y = from_value(5.0);

    // capturing::capturing();
    closure_traits::closure_traits();
}
