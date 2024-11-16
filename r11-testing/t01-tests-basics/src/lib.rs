fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn is_greater(a: i32, b: i32) -> bool {
    a > b
}

fn in_range(val: i32, min: i32, max: i32) -> bool {
    if val < 0 {
        panic!("{val} can not be less than 0");
    }

    if val < min || val > max {
        panic!("{val} is not in range [{min}, {max}]");
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_greater;

    use super::*;

    #[test]
    fn simple_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn simple_fail() {
        panic!("Want it to fail");
    }

    #[test]
    fn is_greater_test() {
        // 'assert!' macro takes a value which evaluates
        // to a boolean.
        assert!(is_greater(5, 4));
    }

    // Testing that a function should panic.
    #[test]
    #[should_panic]
    fn guess_test_1() {
        in_range(5, 10, 20);
    }

    // The 'should_panic' will pass the test,
    // even if it panics for some other reason
    // than we intended.

    // We can pass a text to 'should_panic',
    // it should pass if the panic caused,
    // contains that text.
    #[test]
    #[should_panic(expected = "in range")]
    // It fails, because the code panic
    // because of a different reason.
    fn guess_test_2() {
        in_range(-5, 10, 20);
    }

    #[test]
    #[ignore] // <-- Mark a test as ignored.
    fn i_am_ignored() {}
}
