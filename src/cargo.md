# Cargo

**Cargo** is rust's build and dependency management system. It is similar to `gradle` and `maven` in Java.
However in comparison to them Cargo is much simpler. For example Cargo uses the
[TOML](https://github.com/toml-lang/toml) file format for it's config, which is written in a declaritive style.

For example you might write:
```toml
[package]
name = "hello_world"
version = "0.0.1"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
snafu = "0.6.8"
serde = "1.0"
# This is a comment ...
```

You can find more information about cargo in the [cargo book](https://doc.rust-lang.org/cargo/index.html)

Some common commands are listed below:

|  Command                 |  Effect   |
| ------------------------ | --------- |
| `cargo new --bin foo`    | Create a new program named "foo" |
| `cargo new --lib foo`    | Create a new library named "foo" |
| `cargo build`            | Build the current project |
| `cargo clean`            | Cleans the build directory |
| `cargo check`            | Checks for complication errors |
| `cargo doc`              | Builds [Rustdocs](./method_signatures.md#Rustdocs) |
| `cargo test`             | Runs tests (Like Junit, but built into the language) |
| `cargo publish`          | Publish the crate to [Crates.io](https://crates.io) |



  * https://blog.rust-lang.org/2016/05/05/cargo-pillars.html
  * Toml format
  * Dependencies
    * Dev-Dependencies
  * Cargo tools
    * Audit
    * Doc test
## Versions
  * Depreciation warning

One old idea that Rust embraces is versioning. Unlike Java (or most other programming languages) in Rust when creating a new project you explicitly specify which edition of the language you wish use. This Book is written to the [2018 edition of the language](https://hacks.mozilla.org/2018/12/rust-2018-is-here/). This edition has different syntax than the 2015 edition, and future editions will change things as well. But backwards compatibility is maintained because the compiler can check which edition is being used and compile to that standard. Because regardless of which edition is being used the code is compiled to native code, code written in any edition can depend on code written in any other edition. This frees the language to evolve and change overtime without being tied down to legacy.

Similarly in Rust all projects are expected to use semantic versioning of the form Major.Minor.Patch where a change to the patch version means no publicly visible changes have occurred, a change to the minor version means changes have occurred but existing code should still work, and a change to the major version means dependant code needs to be re-written. Changes that break backwards compatibility are also OK prior to a 1.0.0 version. 

The Rust language itself also versioned in this way. This is actually how Java was originally versioned. They released 1.2, 1.3, 1.4, and 1.5. But then they decided that they could drop the leading 1 because they were not going to break compatibility. Just looking at the version number you can see that Rust releases a lot faster than Java. The Language is on a six-week release cycle so it's constantly improving. 

This rapid release cycle is made possible because it is well factored in well tested. Most of what would be considered the standard library in another language is packaged and released separately. The developers even have a tool which will build all publicly available code against a new version of the language before it is released.

Unlike Java, Rust has a very small standard library. Instead it provides powerful and flexible primitives, that allow many features you wouldn’t expect to be developed as libraries. This works because Rust has a built in dependency management system called Cargo. 

## Dependencies

Dependencies can be added to your project by simply addin the approprate line in Cargo.toml. 
To find crates you can depend on and the line to add them to your config file go to [crates.io](https://crates.io/).

Unlike Java, you don't need to worry about transitive dependencies, it is not possible to endup accedently directly using their code,
and "version conflicts" are not an issue, because multiple versions can be in the same application at the same time without ambiguity. 

Below is a list of common dependencies and their function.

|  Crate    |   Function   |
| --------- | ------------ |
| [Serde](https://serde.rs/) | Serialization and Deserialization |
| [Clap](https://clap.rs/) | Command line argument parsing. (See also [structopt](https://crates.io/crates/structopt) which automates even more |
| [Simple-signal](https://crates.io/crates/simple-signal) | Unix signal handling |  
| [Bindgen](https://rust-lang.github.io/rust-bindgen/) | Automatically generates Rust FFI bindings to C |
| [Itertools](https://crates.io/crates/itertools) | Extra iterator adaptors and methods |
| [Rayon](https://crates.io/crates/rayon) | Simple lightweight data-parallelism library |
| [Faster](https://crates.io/crates/faster) | SMID made safe and simple using iterators |
| [Packed-simd](https://rust-lang-nursery.github.io/packed_simd/packed_simd/) | Architecture independent SIMD |
| [Crossbeam](https://github.com/crossbeam-rs/crossbeam) | Tools for concurrent programming |
| [Tokio](https://tokio.rs/) | A non-blocking IO framework |
| [Diesel](https://diesel.rs/) | A compile time validated ORM query buider |
| [Rocket](https://rocket.rs/) | A web server that uses compile time checks to prevent vunribilites (directory traversal, SQL injection, csrf, css, remote code execution, misconfiguration) and verifies authentication, authorization, and input validation logic. |
| [Maud](https://maud.lambda.xyz/) | Compile time validated HTML templates |
| [Rand](https://crates.io/crates/rand) | Random number generators |
| [Quickcheck](https://crates.io/crates/quickcheck) | Randomized unit testing |
 

  * Logging of events for metrics
  * Rtfm for embedded systems. Also Singleton. And svd2rust
  * Instead of rolling your own graph: Petgraph or an ECS 
  
  * Rustfmt/racer/rustfix
  * Rustup
  * Cargo
  * Rr
  * Fuzzers
    * Cargo fuzz uses an address checker to verify uninitialized memory is not being read.
    * Angora - Still a work in progress, but should eventually be the best fuzzer.
  * Habitat
  * Servo
  * redox
  * Ripgrep

Below are some common dependencies and a brief description of what they do:
High precision math and units
IoUtils
Builder equals, hash, and serializable
Log
slog
Similar to Junit
Benchmark / criterion
Example testing
Quick check / propcheck
Checksyle
Findbugs
Lint (clippy)
Fuzzers: afl / hongfuzz / cargo fuzz

## Rustc
If you've been programming in Java for a long time you're probably used to compiler-errors that you just look at to get the line number and then go to the that line and see what's wrong. IDEs have mostly even obsoleted this by just underlining the problem, which is usually sufficient to work out the problem. Rust goes well above and beyond this. It has very clear and detailed error messages explaining exactly what the problem is some of which even suggest Solutions what's more each error message has a number associated with it but you can look up online and see if full documentation as to why that error message occurs examples of it occurring how to avoid it and how to structure code so is to prevent it from being a problem.
