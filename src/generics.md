# Generics

  • Basic syntax
    ◦ Functions
    ◦ In structs
    ◦ In traits
    ◦ Defaults
    ◦ Associated types - FromStr provides a good example. 
Non-trivial example to show assignment + use
  • Strongly Prefer associated types on traits
    ◦ Better declaration syntax with multiple values
    ◦ Real name for type
    ◦ Named when used so more clear what is what vs order
    ◦ No call site turbofish
    ◦ Allows certain patterns, like continuation token. (Doesn't go up the stack)
    ◦ Makes faster code (one impl, static dispatch)
    ◦ It avoids “generics hell”
    ◦ If you are the one implementing the trait, you can add new types without breaking users
    ◦ Can also have default.
    ◦ Does not work for things like Add where you want many impls depending on both Lhs and rhs
    ◦ Only case for normal generic is for overloading
  • In Rust Generics are called “Type Parameters” because that’s sortof what they are. They are parameters to the function that are types, as opposed to objects. Rust actually handles them this way as opposed to just using them to generate an error if type don’t match. For example in Rust you can define an interface with a ‘new’ method on it. __ Notice that this method does not take a ‘self’ parameter. This means you can’t invoke it on an object. It is what Java would call a static method. However unlike Java you can still invoke this function from inside of a static method. For example __. Here we have a struct that implemented out trait _ and then later when we invoked the generic method _. It used the Type Parameter to invoke the function on the correct type. In this case that parameter did not come from any of the input arguments because there weren’t any. It was inferred from the return type which the compiler obtained by looking at the type of the value we were assigning the call to. 
    ◦ https://bluejekyll.github.io/blog/rust/2017/08/06/type-parameters.html 
  • Conditional trait implementation
  • Traits with methods contingent of a generic implementing a specific trait.
    ◦ IE: cell has take if T is default or get if T is copy. Or a wrapper implements order if the thing it wraps does.
  • Syntax for ‘where’ vs ‘:’ (also ‘+’)
  • Generics + Builders
  • Self type 
    ◦ Can return an newly created instance of a generic type. (collect on iterator)
  • Generics and dyn
  • static vs dynamic dispatch
    ◦ Dyn keyword
  • Rust avoids generics hell
    ◦ Ownership and borrowing save the day
    ◦ https://doc.rust-lang.org/nomicon/subtyping.html 
## Type Inference

Rust generics look a lot like Javas. For example _ vs _.
But Rust also has some shortcuts to make things easier. For example a type can be declared like this _.
Here the character ‘_’ tells the compiler “You figure out what's supposed to go here, I'm not going to bother to type it.” obviously this only works in places where there is enough content to work that out.

  • Declare on the right
  • Where clause

Because traits tend to be small and only have a few methods it's more common in Rust to have a parameter that implements multiple of them. In Java you would write it like this __ the equivalent in rust is __.

In the case of a single trait/interface. In Java you might write __ where 'T extends’ is used to indicate there is some type T but the function doesn't care about the exact type of the object, provided it implements the specified interface. In Rust there is a shorthand for this using the 'impl’ keyword. __. This is most commonly used for return values for example __ specifies that this function is returning a _ but without providing the exact type. This is especially common when a function is returning a function. (Like a callback for example) __ here the function is specifying that it is returning a function with a particular signature but without specifying exactly which function it is returning. So you can think of impl as meaning “some implementation of this trait”. (It doesn't matter which) ‘impl’ only can refer to one trait. So if you need to use multiple, you'll have to use a where clause _ or an alias _.

Optimizer: I can actually make methods invoked on an ‘impl’ parameter a lot faster. So take advantage of that.