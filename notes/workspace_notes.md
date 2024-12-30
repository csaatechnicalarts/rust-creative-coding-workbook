# Rust Workspaces

[Stack Overflow: Rust Multiple Binaries](https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo) A good discussion prompted by the need to have multiple binaries and libraries in separate directories, all under one cargo package. Shepmaster mentions Cargo workspaces, which sounds like the best approach to the problem.  

[Rust Programming Language: Workspaces.](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) Lengthy explantion with clear examples.

`$> cargo run` - Builds and runs the binary `[root]/src/main.rs`.

`$> cargo build -p <workspace-name>` - Builds and runs the binary `[root]/workspace-name/src/main.rs`.

`$> cargo run -p <workspace-name> -- <cmd-line parameters>` - Executes the workspace binary, passing parameters to it.