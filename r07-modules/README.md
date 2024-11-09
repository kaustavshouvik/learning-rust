1. Crate -> Smallest amount of code the compiler considers at compile time.
   - A single file is also considered to be a crate.
   - Crate contain modules.
   - Only two type of crates:
     - Binary: Have a main function and gets compiled to an executable.
     - Library: Code defining functionality.
2. Package -> A bundle of crates.
   - `Cargo.toml` defines how to build those crates.
   - Can contain many binary crates and optionally one library crate.
     Can contain at max only one library crate.
   - A package must have at least one create of any type.

<pre>
  Package
    |
    -- Crate
          |
          --- Module
          --- Module
    --Crate
          |
          --- Module
</pre>

3. `cargo new <thing>` create a package.
   - A package with only `src/main.rs` contains only a binary crate.
   - A package with only `src/lib.rs` contains only a library crate.
   - A package with both `src/main.rs` and `src/lib.rs` contains both a binary and library crate.
   - A package with multiple binary crates is defined in `src/bin` directory.
     Each file is a separate binary crate.
