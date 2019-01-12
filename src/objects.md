# Objects

  • Members 
    ◦ Constants
  • Methods
    ◦ Functions vs methods (implicit borrows)
  • Visibility
  • Static methods
  • Constructors
    ◦ Like Java, you can never have a partially constructed / uninitialized object
    ◦ Can return a different object from “new” this is useful for having things like a builder.
    ◦ From_ naming convention for constructors 
  • Traits vs Interfaces
    ◦ Drop vs TryWith object vs finalizer
  • Add methods outside
  • Difference between impl foo for bar vs impl<t:bar> foo for t
    ◦ Impl foo is a trait it just doesn't have a name.
  • Traits and emulate overloading but in an extensible way
  • Use types to provide static distinctions (because types have no overhead)
  • Common traits
    ◦ convert
    ◦ copy
    ◦ AsRef / asMut
    ◦ From / tryfrom  /  Into / tryInto
      ▪ Major advantage is don’t need to actually know type.
      ▪ Automatically mirror one another so don’t need to specify in two locations.
      ▪ Types automatically convert to themselves.
      ▪ Great example of generic implementations as your code can benefit from methods you did not write.
      ▪ Orphan rules mean you can only implement one.
      ▪ Notice that into() can just be invoked and it figures out which method to call based on what the return type is.
        • This is much more powerful than Java’s method overloading because you cannot have identical signatures do different things. 
    ◦ fromString
    ◦ Debug (always implement) 
    ◦ Partial order
    ◦ Hash
    ◦ Display
    ◦ Error
    ◦ Default
    ◦ Autotraits: sync, send, sized, ?sized
  • Almost all of these are written in standard rust. Including thing like iterator which will be covered in the next chapter. These automatically add a LOT of premade functions for your custom made types that you don’t have to write. For the traits supporting #derive this is even easier, because you don’t even have to do anything besides type the name. Even more importantly because everything is just standard rust, and there are no special hooks in the compiler for any of these types. This means in your own code you can build abstractions that are just as powerful. <Even better this is all done at compile time so there is no overhead>

The in Java methods have access to an implicit variable ‘this’ which is the object on which the method was invoked. In Rust ‘this’ is called ‘self’. In Java there are methods declared with the keyword ‘static’ which do not have access to ‘this’ because they are not associated with any particular instance. In Rust the ‘self’ parameter is explicitly declared as the first argument to a method. So leaving it out is analogous to declaring a method ‘static’ in Java. Similar to Java such a method is invoked from the type itself. The following functions are equivalent __ and __. 


Rust is an Object Oriented language, but it is not a class oriented language. So unlike in Java not all of the methods on an object are located in the same block of code. They are broken out across multiple traits. In many cases there are internal/related structs, enums, and traits which allow for greater code reuse. For example below is a table of common traits __(ndex, intoiter, etc___. Each of these is small and easy to implement. Each one appears on a lot of types you wouldn't necessarily guess right away. For example _, and _. The advantage is each one unlocks a lot of flexibility because there is a lot of code that takes advantage of them. See __ or __ or __ as there is a lot of functions built around iterators. This pattern holds generally in Rust. Classes are not monolithic. Instead they have many small components that hook into each other to provide greater code reuse. 


Traits allow can have implementation methods on them like Java interfaces (as of Java 8). For example __ is the same as __. Where the trait is defining the method _ but leaving the methods _ and _ up to the implementer of the trait.

Traits are a concept in Rust that are very similar to an interface in Java. The main difference being that by convention Traits tend to have very few methods, usually just one or two each. As such APIs tend to be a bit finer grained but you may deal with more of them.

Below are some common interfaces in Java and their Rust equivalents:
__
