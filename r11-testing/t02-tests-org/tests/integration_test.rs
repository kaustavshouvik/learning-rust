mod common;

use t02_tests_org::{op, CalcOp};

// Each module is separate crate,
// so we have to bring in the thing
// we want to test.
//  -> Just like a user would.

// Note that integration tests only runs
// when unit tests passes.

#[test]
fn calc_add_test() {
    common::setup();

    let res = op(CalcOp::Add(5, 5));
    assert_eq!(res, 10);
}
