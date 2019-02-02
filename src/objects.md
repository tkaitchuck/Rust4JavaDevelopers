# Objects
  * Members 
    * Constants
  * Methods
    * Functions vs methods (implicit borrows)
  * Visibility
  * Static methods
  * Constructors
    * Like Java, you can never have a partially constructed / uninitialized object
    * Can return a different object from “new” this is useful for having things like a builder.
    * From_ naming convention for constructors 
  * Traits vs Interfaces
    * Drop vs TryWith object vs finalizer
  * Add methods outside
  * Difference between impl foo for bar vs impl<t:bar> foo for t
    * Impl foo is a trait it just doesn't have a name.
  * Traits and emulate overloading but in an extensible way
  * Use types to provide static distinctions (because types have no overhead)
  * Common traits
    * convert
    * copy
    * AsRef / asMut
    * From / tryfrom  /  Into / tryInto
      * Major advantage is don’t need to actually know type.
      * Automatically mirror one another so don’t need to specify in two locations.
      * Types automatically convert to themselves.
      * Great example of generic implementations as your code can benefit from methods you did not write.
      * Orphan rules mean you can only implement one.
      * Notice that into() can just be invoked and it figures out which method to call based on what the return type is.
        * This is much more powerful than Java’s method overloading because you cannot have identical signatures do different things. 
    * fromString
    * Debug (always implement) 
    * Partial order
    * Hash
    * Display
    * Error
    * Default
    * Autotraits: sync, send, sized, ?sized
  * Almost all of these are written in standard rust. Including thing like iterator which will be covered in the next chapter. These automatically add a LOT of premade functions for your custom made types that you don’t have to write. For the traits supporting #derive this is even easier, because you don’t even have to do anything besides type the name. Even more importantly because everything is just standard rust, and there are no special hooks in the compiler for any of these types. This means in your own code you can build abstractions that are just as powerful. <Even better this is all done at compile time so there is no overhead>

The in Java methods have access to an implicit variable ‘this’ which is the object on which the method was invoked. In Rust ‘this’ is called ‘self’. In Java there are methods declared with the keyword ‘static’ which do not have access to ‘this’ because they are not associated with any particular instance. In Rust the ‘self’ parameter is explicitly declared as the first argument to a method. So leaving it out is analogous to declaring a method ‘static’ in Java. Similar to Java such a method is invoked from the type itself. The following functions are equivalent __ and __. 


Rust is an Object Oriented language, but it is not a class oriented language. So unlike in Java not all of the methods on an object are located in the same block of code.



# Traits
  * Default methods
  * Traits can inherit
  * Associated types
    * Needs an example. (Specs does this)
    * Associated constants
  * Marker traits. Like serializable in Java. Can be used to enforce certain semantics like that a particular behaviour / handling is desirable. 
    * Copy trait
    * Can also be made mutually exclusive which is useful for making the compiler enforce invariants. 


Traits allow can have implementation methods on them like Java interfaces. For example __ is the same as __. Where the trait is defining the method _ but leaving the methods _ and _ up to the implementer of the trait.

Traits are a concept in Rust that are very similar to an interface in Java. The main difference being that by convention Traits tend to have very few methods, usually just one or two each. As such APIs tend to be a bit finer grained but you may deal with more of them.




## Implementing traits

In Rust the implementation of traits is declared in a separate block from the declaration of the member variables. While this might seem strange coming from Java, it is actually a very important feature. Because methods are not located in the object itself the language doesn't need to distinguish between 'primitives’ and 'objects’ like Java. This is why Java needs Integer and int, Long and long, and Double and double. 

So in Rust an i32 can be a key in a map without needing to be wrapped. This is also useful because they don't have to be in the same file. So you can for example declare a trait and the supply an implementation for an existing type. For example here is a trait for objects that can be doubled:
```rust
trait doubleable {
  fn double(self : Self) -> Self;
}

// And we can implement this for i32:

impl doubleable for i32 {
  fn double(self : i32) -> i32 {
    2 * self
  }
}

// And now we can invoke this like any other method:

assert_eq!(10, 5.double());
```
It's worth noting that there is never any ambiguity about where the implementation of a particular trait is. This is because it is only allowed to be in one of two places, where the trait is defined or where the type is defined. Because circular dependencies are not allowed and the definition will need to depend on both, the one that depends on the other must contain the implementation. This is referred to as “the orphan rule”. It appears in couple of places, aside from making method calls unambiguous, it also ensures things work as you might intuitively expect and means that unlike some languages (*ahem* Scalla) the behavior of code can't be altered simply by adding an import statement.

## Overloading

Java has method overloading which works like this:
```java
class Processor {
  public void process(Foo item) {
    //...
  }
  public void process(Bar item) {
    //...
  }
}
```
in Rust the equivlent would be:
```rust
mod processor {
# struct Foo; struct Bar;
  struct Processor;
  trait Process<Item> {
    fn process(&self, item : Item);  
  }
  impl Process<Foo> for Processor {
    fn process(&self, item : Foo) {
     //...
    }
  }
  impl Process<Bar> for Processor {
    fn process(&self, item : Bar) {
      //...
   }
  }
}
```
In Rust the methods are defined on the impl of the traits. This might seem a bit verbose, but traits are more powerful than simple overloading that you can do in Java. It also has the advantage that the implementations don't need to be located where `Processor` is declared. A new type can add it's own overload and implement it by for example by delegating to an existing implementation. Additionally in Rust you can overload functions with identical signatures or that differ by only a generic or a return type. each of those is associated with a trait. In Java you can never implement the same interface for two different generic types. For example:
```java
class Processor {
  void processList(List<Foo> items);
  void processList(List<Bar> items); //Ooops..
}
```
Where as rust can just add:
```rust,ignore
impl Process<Vec<Bar> for Processor {
  fn process(&self, item : Vec<Bar>) {
    //...
  }
}
impl Process<Vec<Foo> for Processor {
  fn process(&self, item : Vec<Foo>) {
    //...
  }
}
```
Instead you have to overload the methods. This means the same class cannot implement an interface for multiple types.
```java
class FastProcessor implements Processor<Foo>, Processor<Bar> { //Oops not allowed...
  //...
}
```
This of course destroys code reusability and prevents you from using interfaces as a means of abstraction because you are always coding to the impl. In Rust the interface is mandatory, which makes it easy to extend in the future and forces you think about the generalization. For example here is some code I wrote in Java __. Notice the overloaded method. In Rust I would have to create an interface that defines what I want. In this case _. This is already a common pattern so I can just use _. Then I can generalize my implementation by writing __. Now not only can I overload this method by adding more implementations, but my callers can too! All they have to do is implement _ like so __.

## The visitor pattern is a hack

Java uses overloading to have a single method that can work for multiple different types. However Java also has what is called “single dispatch”. This means if you want a function to work generically for all implementations of a given interface but a different function for each one, you need to use the Visitor pattern:
```java
class Processor {
  public void process(Foo item) {
    //...
  }
  public void process(Bar item) {
    //...
  }
}
interface Processable {
  public void process(Processor processor);
}
class Foo implements Processable {
  public void process(Processor processor) {
    processor.process(this);
  }
}
class Bar implements Processable {
  public void process(Processor processor) {
    processor.process(this);
  }
}

Processor processor = //...
List<? extends Processable> items = //...
for (Processable item : items) {
  // This will invoke the overloaded Processor.process(item) with the correct type.
  item.process(processor); 
}
```
You’ve probably written code like that a few times, and it generally works. It's certainly better than a single function with a large switch statement. But it still involves a lot of boilerplate code because each impl needs to write the visit function.
This is even more problematic if you don't own they types that need to implement visit() because then you can't write such a method. So your back to either writing a lot of conditional logic in a large function to sort them out, or you have to wrap the types with your own type.

In Rust none of that would be needed. You can add an implementation of an trait that is type specific. 
```rust
mod processor {
# pub struct Foo; pub struct Bar;
  pub struct Processor {
    //...
  }
  pub trait Processable {
    fn process(&self, processor : &Processor);  
  }
  impl Processable for Foo {
    fn process(&self, processor : &Processor) {
      //...
    }
  }
  impl Processable for Bar {
    fn process(&self, processor : &Processor) {
      //...
    }
  }
}
//...
use crate::processor::*;
pub fn main() {
  let p : Processor = //...
# Processor {};
  let items : Vec<Box<dyn Processable>> = //...
# vec![];
  for item in items {
    item.process(&p);
  }
}
```
In the above example notice the `Box<dyn Processable>`. `dyn` is equivlent to the `? extends` in Java example above. It indicates there is more than one implementation. IE: the type is dynamic and the actual implementation won't be known until runtime. The box is actige as a 'fixed size container' so that the itmes in `Vec` can all be the same size. 

In Java this "boxing" is done for all collections all the time, which is why primitives can't be placed in them. There is syntatic sugar for this called "autoboxing" which converts `int` into `Integer`. In Rust boxing is explicet.


# An example

Often classes are implemented with multiple traits. For example, Java's UUID class would be in a file named UUID.java and look like this:
```java
import java.util.Random;

public final class UUID implements Serializable, Comparable<UUID> {}
  private static final long serialVersionUID = -4856846361193249489L;

  private long mostSigBits;

  private long leastSigBits;

  public UUID(long mostSigBits, long leastSigBits) {
    this.mostSigBits = mostSigBits;
    this.leastSigBits = leastSigBits;
  }
  
  public int compareTo(UUID o) {
    //...
  }
  public boolean equals(Object obj) {
    //...
  }
  public int hashCode() {
    //...
  }
  public String toString() {
    //...
  }
  public static UUID fromString(String name) {
    //...
  }
  public static UUID randomUUID() {
    //...
  }
}
```
In Rust the equivlent would be in a file named UUID.rs and look like this:
{{#playpen object_example.rs}}

The major difference between the Java version and the Rust version is that instead of being in a single `class` block where as the Rust implementation is split into three a `struct` block which contains the fields, an `impl` block which contains all of the class speffic methods, and separate `impl` blocks for each of the different traits that are implemented. 

In the Java case, there are two interfaces implemented `Serializable` and `Comparable`, where as `equals`, `hashCode`, and `toString` aren't interfaces, but rather are inherited from Object. Rust breaks things down a little more finely. Some of the `Serialize` and `Deserialize` are split into separate traits. In this case they appear in the `derive` "attribute". An attribute is like a Java annotation. `derive` Automatically generates boilerplate implementations of common traits. In addition to these `Eq` is the analogous trait for `equals`, `hash` provides `hashCode`. In addition to these `PartialEq` and `PartialOrd` provide the equivlent of `equals` and `Comparable` without the requirement that all instances be differentiated. IE: Unlike `eq` if two instances can have member variable that is different but they are still considered the same for the purposes of equality, or in the case of `PartialOrd` unlike `Ord` there can be instances that can't be compared such as a floating point NaN. `Debug` is a trait to print a string representation for the purposes for debugging much like Java's `toString`. The traits `From` and `Into` are common traits for converting between different types. In this case if fills the same role as `fromString` implementatation in Java.

## Patterns
Because traits tend to be small and only have a few methods it's more common in Rust to have a parameter that implements multiple of them. To pick a simple class to see how this works in practice let's compare Java's `AtomicBoolean` to Rust's `AtomicBool`

|Attribute      | &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Java's AtomicBoolean | &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Rust's AtomicBool |
| ------------- |--------------------:  |-----------------:  |
| Total lines   |    359                |    532             |
| Lines of docs |    172                |    361             |
| Lines of code |     72                |     63             |
| Public methods|     20                |     14             |
| Interfaces/traits implemented| 1      |      5             |
| Additional traits automatically implemented because they were defined elsewhere | n/a | 8 |
| Total methods invokable| 20           |      32            |

So while the actual implementations are very similar (There are only so may ways to implement an atomic boolean and only so many things you can do with it), a large chunk of the Rust implementation came 'for free' to the author of `AtomicBool` bacause 18 of the 32 methods were implemented Automatically by virtue of other traits/methods that were defined. In addition to this other developers can add their own interfaces and implementations. For example, the one interface Java's `AtomicBoolean` implements is `Serializeable`. In Rust, the dependency goes in other direction. `AtomicBool` is a basic type and the author of the serialization can provide their implementation for serializing it. So any code that depends on both serialization and on atomicBoolean will see even more methods (those implemneted by the author of serialization, as well as any further traits it gets 'for free' because of the ones it now implementes).

As a result, it really pays to keep your own traits small and focused, and aggressivly implement common traits for your types.

## Common Traits

This fine grained declaration of traits allows for greater code reuse. For example below is a table of common traits

| Trait      | Similar Java method | Description            |
|:-----------|---------------------|------------------------|
|PartialEq   | Object.equals | Defines if two instaces are semantically equal. |
|Eq          | Object.equals | Defines if two instance are equal and can differentiate between all instances |
|PartialOrd  | Comparable.compareTo | Defines >, >=, <, and <= operators |
|Ord         | Comparable.compareTo | Defines ordering, and can order all unique instances |
|Index       | List.get or [x] operators | Defines indexing IE: [x] much like a Java array |
|IntoIterator| Iterable.iterator | Defines a method to construct an iterator |
|Iterator    | Iterator | Allows iterating over a collection |
|Debug       | Object.toString | Creates a string representation of an object for debugging |
|Display     | Object.toString | Creates a human readable representation of an object |
|From        | N/A | A generic conversion function to instantiate one type from another |
|Into        | N/A | A generic conversion function to convert one type into another |
|Copy        | Java primitives | A type that is "passed by value" meaning it is copied each time it is assigned to a new value |
|Clone       | Cloneable.clone | A function that makes a copy of an object |
|FromStr     | N/A | Constructs an object from a string |
|ToString    | Object.toString | Converts an object into a string |
|Default     | A zero argument constructor | Instantiates a default version of an object |
|Error       | Exception | An exeption |
|Hash        | Object.hashCode | Used when storing an item in a HashMap or HashSet |
|Optional    | Optional | Either an item or `none` indicating its absence |

Each of these is small and easy to implement. Each one appears on a lot of types and is accepted in a lot of common functions. So each one a class can implement unlocks a lot of flexibility. See __ or __ or __ as there is a lot of functions built around iterators. This pattern holds generally in Rust. Classes are not monolithic. Instead they have many small components that hook into each other to provide greater code reuse. 