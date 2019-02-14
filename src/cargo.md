# Cargo
  * https://blog.rust-lang.org/2016/05/05/cargo-pillars.html
  * Toml format
  * Dependencies
    * Dev-Dependencies
  * Cargo tools
    * Audit
    * Test
    * Doc test
## Versions
  * Depreciation warning

One old idea that Rust embraces is versioning. Unlike Java (or most other programming languages) in Rust when creating a new project you explicitly specify which edition of the language you wish use. This Book is written to the 2018 edition of the language. This edition has different syntax than the 2015 edition, and future editions will change things as well. But backwards compatibility is maintained because the compiler can check which edition is being used and compile to that standard. Because regardless of which edition is being used the code is compiled to native code, code written in any edition can depend on code written in any other edition. This frees the language to evolve and change overtime without being tied down to legacy.

Similarly in Rust all projects are expected to use semantic versioning of the form Major.Minor.Patch where a change to the patch version means no publicly visible changes have occurred, a change to the minor version means changes have occurred but existing code should still work, and a change to the major version means dependant code needs to be re-written. Changes that break backwards compatibility are also OK prior to a 1.0.0 version. 

The Rust language itself also versioned in this way. This is actually how Java was originally versioned. They released 1.2, 1.3, 1.4, and 1.5. But then they decided that they could drop the leading 1 because they were not going to break compatibility. Just looking at the version number you can see that Rust releases a lot faster than Java. The Language is on a six-week release cycle so it's constantly improving. 

This rapid release cycle is made possible because it is well factored in well tested. Most of what would be considered the standard library in another language is packaged and released separately. The developers even have a tool which will build all publicly available code against a new version of the language before it is released.

Unlike Java, Rust has a very small standard library. Instead it provides powerful and flexible primitives, that allow many features you wouldn’t expect to be developed as libraries. This works because Rust has a built in dependency management system called Cargo. 

Cargo is very will integrated into the language. You can think of Cargo as being similar to Gradle and Maven in Java. Each project has a `Cargo.toml` file. This is where ….

## Common dependencies

Below is a list of common dependencies and their function.

|  Crate    |   Function   |
|-----------|--------------|
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


  * Logging of events for metrics
  * Rtfm for embedded systems. Also Singleton. And svd2rust
  * Instead of rolling your own graph: Petgraph or an ECS 


  * Rustfmt/racer/rustfix
  * Rustup
  * Cargo
  * Rr
  * Quick check
  * Fuzzers
    * Cargo fuzz uses an address checker to verify uninitialized memory is not being read.
    * Angora - Still a work in progress, but should eventually be the best fuzzer.
  * Habitat
  * Servo
  * redox
  * Ripgrep


Below are some common dependencies and a brief description of what they do:
Cargo
Rustup
Rustc
Random
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
