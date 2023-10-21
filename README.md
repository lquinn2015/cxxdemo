### Demo CXX
This repo simply is a demo on using cxx to use rust within a c++ program. 
It is really simple essentially it works like this.

## From Rust rcalc
To start with rcalc is a rust lib check its crate types in Cargo.toml = ["rlib", "staticlib"].
We care about the staticlib which is what we link to. To generate this lib we do 
cargo build --release look at the `rcalc/makefile`  this will invoke the build.rs

# build.rs -- Generates cxx::bridge 
The build.rs generates 2 directories into `target/cxxbridge/`  `rust` and `rcalc` these
directories contain the cpp code required to work with our ffi module in lib.rs. In particular
the lib.rs.h is in `target/cxxbridge/rcalc/src/` is where our ffi module is thus header is built
to there. Including this header gives you bridge access!

# ffi module 
The mod ffi is decled with #[cxx::bridge(namespace=rcalc)]. You don't need to have a namespace
but the idea is that you probably want one for the c++ project you are injecting into. 
I export 3 global functions add,sub and grab_calc. grab_calc returns me a Boxed handle
to a Calc object. This object can invoke mul!

# C++ land
You simply need to include `rust/cxx.h` and `lib.rs.h` kind of a bad name but thats life. 
Than you can use those types and objects

