pub fn panicking() {
    // When code panics:
    // 1. It cleans up the resources.
    // 2. Prints some details.
    // 3. Exits.

    // When panic caused?
    // 1. Calling panic!() macro manually.
    // 2. Calling some other code which panics.

    // panic!("Manual panic");

    let v = vec![1, 2, 3];
    v[99];

    // To view a backtrace:
    // Run with the env variable -> RUST_BACKTRACE
    // with any value other than 0.

    // Running with backtrace enabled also requires
    // 'debug symbols' to be enabled, which is enabled
    // by default (running without the '--release' flag).
}
