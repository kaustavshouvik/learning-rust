1. Integration tests go inside `tests` directory,
   at the same level as `src` directory.
2. Each file in `tests` directory is a separate crate,
   we need to bring in the thing we need to test in the scope.
   - Just like a user would.
3. There is now 3 sections of tests: unit, integration, doc.
   - Each section only runs if the all previous section passes.
   - Each file in `tests` gets its own section.
4. To have some common functionality:
   - `tests/common.rs`: Not the way, rust thinks it is a test file.
   - `tests/common/mod.rs`: This is the way.
5. How do you test binary crates?
   - You don't.
   - Rust's rationale: Binary crates should be small and should import
     functionality from a library crate, and since a library crate
     can be tested, you do that and the small amount of code
     in the binary crate doesn't really need to be tested.
