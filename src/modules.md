# Modules

  * Rules of modules and files. (See the rust book)
  * Inline modules.
  * Testing modules.

In Java code is distributed in **Jars**, there is a directory hierarchy which orginizes the code within them into multiple Java **packages**, within a package there are many files, each of which holds a corisponding **class** which intern holds member variables and has methods. 

In Rust the situation is similar, code is distributed in **crates** inside of which there is a directory hierarchy which orginizes the code in them into multiple **modules**. In Rust modules are hierarchial, so a module could have a submodule that is a whole directory tree of sub modules, on just a single file which contains the module. Inside of the file, there can be member variables declared in a struct and methods defined to operate on them.

As an example a crate might have the following structure:
```
src/lib.rs
src/image_processor.rs
src/image_processor/jpeg_decoder.rs
src/image_processor/png_decoder.rs
src/analyzer.rs
```
Here `lib.rs` is the entrypoint for the crate, meaning it is a library that other crates depend on. If the crate was a program intended to be executed directly, there would instead be a `main.rs` in the `src` directory. All of the source files to be included go under the `src` directory where as tests go under a `tests` directory. 

Inside of `lib.rs` the code might import the image processor module by:
``use image_processor::*``
This uses a relitive path to resolve items. It is also possible to provide an absolute one by:
``use crate::image_processor::*``
Here the keyword `crate` refers to the root from anywhere within the crate. Similarly there are keywords for `self` to refer to the current module and `super` to refer to the parent module.

Modules affect visability. By default methods are visable within a module and any submodule. (Note this this is different from Java's default visibility which is not hierarchical) So functions in `lib.rs` (assuming they are not in a sub-module) can be seen by any code in the crate. Where as those in `image_processor.rs` can be seen by `jpeg_decoder.rs` and `png_decoder.rs` but not `analyzer.rs` and items in `analyzer.rs` cannot be seen from any other module. To increase this visability one can add the the keyword `pub` in front of a function definition. This works just like `public` in Java. It means that the function can be invoked from anywhere. Additionally it is possible to add `pub(crate)` which makes the item visiable to everything within the crate, but not outside of it. It's also possible to be more explicet and write `pub(in test_module)` to make the module visiable to code in `test_module`.

These rules are quite flexable, as they allow for very clean separation of 'internal' and 'external' concepts. The one thing that is a little supprising at first is that there is no equivlent of Java's `private`. However this is obtained automatically by simply putting things into their own module. In the above example anything in `jpeg_decoder.rs` would be effectivly private unless it was explicetly declared `pub`. 

It’s a good idea in Rust to follow the same design principle that a lot of Java code uses: The Single responsibility principle. Any given chunk of code should have one and only one responsibility. All the code with that responsibility should be located in single file and almost always, that corresponds with a single public structure. (Though there may be several private ones). Java formalizes this by having one (outer) class per file. In Rust you should do the same thing with modules. Each module should be in it’s own file, and have a single, clearly defined responsibility, which means in all likelihood a single public structure (and any subspecies).