# Getting Started with Raylib-C

For Ubuntu, follow this [tutorial](https://github.com/raysan5/raylib/wiki/Working-on-GNU-Linux) to download, build and install raylib.

Once raylib is installed (default shared library and header files are in /usr/local), build your code:

`$> export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib`
 
`$> gcc -o $(NAME_PART).exe $(FILE_NAME) -lraylib -lGL -lm -lpthread -ldl -lrt -lX11`


# Getting Started with Raylib-Rust

Before everything else, clone raylib source code from Github then compile and install the library to your system (see the section above). Cargo will need the raylib library as a prerequisite, sourced through `LD_LIBRARY_PATH`, to build the `raylib-sys` crate.

Make sure the necessary dependies are in place within `Cargo.toml` and you're ready to go. 
```
[dependencies]
raylib = { version = "5.0" }
```

See the [`raylib` crate](https://docs.rs/raylib/latest/raylib/) for sample code:
