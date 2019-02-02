# Modules

  * Rules of modules and files. (See the rust book)
  * Inline modules.
  * Testing modules.

It’s a good idea in Rust to follow the same design principle that a lot of Java code uses: The Single responsibility principle. Any given chunk of code should have one and only one responsibility. All the code with that responsibility should be located in single file and almost always, that corresponds with a single public structure. (Though there may be several private ones). Java formalizes this by having one (outer) class per file. In Rust you should do the same thing with modules. Each module should be in it’s own file, and have a single, clearly defined responsibility, which means in all likelihood a single public structure (and any subspecies).