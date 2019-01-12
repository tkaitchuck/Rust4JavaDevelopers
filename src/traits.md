# Traits
  • Default methods
  • Traits can inherit
  • Associated types
    ◦ Needs an example. (Specs does this)
    ◦ Associated constants
  • Marker traits. Like serializable in Java. Can be used to enforce certain semantics like that a particular behaviour / handling is desirable. 
    ◦ Copy trait
    ◦ Can also be made mutually exclusive which is useful for making the compiler enforce invariants. 
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
# struct Foo; struct Bar;
struct Processor;
trait Processable {
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
let processor = //...
# Processor;
let items : Vec<Box<dyn Processable>> = //...
# vec![];
for item in items {
  item.process(&processor);
}
```
In the above example you may have noticed the 'dyn’ keyword. This is similar to `? extends` in Java. It indicates there is more than one implementation. IE: the type is dynamic and the actual implementation won't be known until runtime.