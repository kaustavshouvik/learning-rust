A simple application to demonstrate cargo workspaces.

1. Just by adding projects, it doesn't mean they are linked somehow.
   - Need to tell a crate to that it's using another crate
     by specifying the path in its `Cargo.toml` file.
   - See `adder`'s `Cargo.toml` file.
2. To run the binary crate -> `cargo run -p adder`.
3. Each package needs to be published independently.
   - Can use the same `-p` option to specify which package to publish.
