1. Gotta have a test module.
2. Tests are rust functions, just annotated with test attribute.
3. Each test runs in a separate thread.
   - The main thread marks a test as failed, if the test thread dies.
   - For example, calling panic macro -> thread dies.
4. Arguments to `assert_eq!()` and `assert_ne!()` must implement
   the following traits:
   - `PartialEq`: It compared left and right values using '==' or '!='.
   - `Debug`: It debug prints the values when assertion fails.
5. When we do `cargo test` it first compiles the project in test mode.
   Then runs the tests.
6. Few options for cargo test:
   - `cargo test -- --test-threads=1`: Run tests sequentially.
   - `cargo test -- --show-output`: Shows output of successful tests.
     By default only output of failed tests are displayed.
   - `cargo test -- --ignored`: Run only ignored tests.
   - `cargo test -- --include-ignored`: Runs ignored tests,
     in additional to non-ignored tests.
