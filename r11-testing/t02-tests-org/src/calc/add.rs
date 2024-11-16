pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests go the same file as the source code.

// The 'cfg()' annotation is used to tell rust that
// this module should only be included if a certain
// configuration is set.

// In this case, the configuration is 'test', which
// is set when running 'cargo test'.

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(10, 10), 20)
    }
}
