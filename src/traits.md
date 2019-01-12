# Traits
  • Default methods
  • Traits can inherit
  • Let's look at how Java and Rust support overloading. In Java you do it one the class like this__. In Rust the methods are defined on the impl of the traits. This might seem a bit verbose, but traits are more powerful than simple overloading that you can do in Java. In Rust you can overload functions with identical signatures or that differ by only a generic or a return type. each of those is associated with a trait. In Java you can never implement the same interface for two different generic types. For example __ Instead you have to overload the methods and not implement an interface. __ This of course destroys code reusability and prevents you from using interfaces as a means of abstraction because you are always coding to the impl. In Rust the interface is mandatory, which makes it easy to extend in the future and forces you think about the generalization. For example here is some code I wrote in Java __. Notice the overloaded method. In Rust I would have to create an interface that defines what I want. In this case _. This is already a common pattern so I can just use _. Then I can generalize my implementation by writing __. Now not only can I overload this method by adding more implementations, but my callers can too! All they have to do is implement _ like so __.
  • Associated types
    ◦ Needs an example. (Specs does this)
    ◦ Associated constants
  • Marker traits. Like serializable in Java. Can be used to enforce certain semantics like that a particular behaviour / handling is desirable. 
    ◦ Copy trait
    ◦ Can also be made mutually exclusive which is useful for making the compiler enforce invariants. 
In Rust the implementation of traits is declared in a separate block from the declaration of the member variables. While this might seem strange coming from Java, it is actually a very important feature. Because methods are not located in the object itself the language doesn't need to distinguish between 'primitives’ and 'objects’ like Java. This is why Java needs Integer and int, Long and long, and Double and double. 

So in Rust an i32 can be a key in a map without needing to be wrapped. This is also useful because they don't have to be in the same file. So you can for example declare a trait and the supply an implementation for an existing type. For example here is a trait for objects that can be doubled:
__
And we can implement this for i32:
__
And now we can invoke this like any other method:
__
It's worth noting that there is never any ambiguity about where the implementation of a particular trait is. This is because it is only allowed to be in one of two places, where the trait is defined or where the type is defined. Because circular dependencies are not allowed and the definition will need to depend on both, the one that depends on the other must contain the implementation. This is referred to as “the orphan rule”. It appears in couple of places, aside from making method calls unambiguous, it also ensures things work as you might intuitively expect and means that unlike some languages (*ahem* Scalla) the behavior of code can't be altered simply by adding an import statement.


Java has method overloading which works like this:
__
This allows you to have a single method that can work for multiple different types. However Java also has what is called “single dispatch”. <Explain concept >
This means if you want a function to work generically for all implementations of a given interface but a different function for each one, you need to use the Visitor pattern:
__
You’ve probably written code like that a few times, and it generally works. It's certainly better than a single function with a large switch statement. But it still involves a lot of boilerplate code because each impl needs to write the visit function.
This is even more problematic if you don't own they types that need to implement visit() because then you can't write such a method. So your back to either writing a lot of conditional logic in a large function to sort them out, or you have to wrap the types with your own type.
__
In Rust none of that would be needed. You can add an implementation of an trait that is type specific. 

In the above examples you may have noticed the 'dyn’ keyword. This is just an indication that the implementation of the type is dynamic and won't be known until runtime. At the moment the keyword is not mandatory as it doesn't really do anything besides add clearly. But it is encouraged and it may be mandated in future Editions of Rust. So we include it.
