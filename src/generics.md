# Generics (aka Type parameters)
Non-trivial example to show assignment + use
  * Strongly Prefer associated types on traits
    * Better declaration syntax with multiple values
    * Real name for type
    * Named when used so more clear what is what vs order
    * No call site turbofish
    * Allows certain patterns, like continuation token. (Doesn't go up the stack)
    * Makes faster code (one impl, static dispatch)
    * It avoids “generics hell”
    * If you are the one implementing the trait, you can add new types without breaking users
    * Can also have default.
    * Does not work for things like Add where you want many impls depending on both Lhs and rhs
    * Only case for normal generic is for overloading
  * Conditional trait implementation
  * Traits with methods contingent of a generic implementing a specific trait.
    * IE: cell has take if T is default or get if T is copy. Or a wrapper implements order if the thing it wraps does.
  * Syntax for ‘where’ vs ‘:’ (also ‘+’)
  * Generics + Builders
  * Self type 
    * Can return an newly created instance of a generic type. (collect on iterator)
  * Generics and dyn
  * static vs dynamic dispatch
    * Dyn keyword


Rust generics look a lot like Javas. When declared on a type:
```rust ,skt-default
# use std::marker::PhantomData;
pub struct LinkedList<T> {
  //...
# not_implemented : PhantomData<T>,
}
```
When instantiated:
```rust, ignore
let connection : HashMap<String, TcpStream> = //...
```
When in a function signature:
```rust ,skt-default
use std::collections::HashMap;
pub fn lookup_emojis(characters : Vec<&str>) -> HashMap<&str, &str> {
  //...
# HashMap::new()
}
```
The biggest difference is that Rust calls them "Type Parameters". [Just as a function may take various parameters, it can
also take a parameter which is a type.](https://bluejekyll.github.io/blog/rust/2017/08/06/type-parameters.html )

Below are some additional examples:
TODO:
* Functions
* In structs
* In traits
* Defaults
* Associated types - FromStr provides a good example. 

## Type Inference

Rust also has some shortcuts to make things easier. For example a type can be declared like this:
```rust ,skt-main
fn get_items() -> Vec<String> {
  vec!["Hello".to_string()] 
}
let to_print : Vec<_> = get_items();
```
Here the character ‘_’ tells the compiler “You figure out what's supposed to go here, I'm not going to bother to type it.”. This allows you to specify a type, while still allowing the generics to be infered. This works and can be used any place where the compiler has enough content to work out the generic value.

## Bounds

In java you might write a function that looks like this:
```java
public static void <T extends Comparable<T>> sort(List<T> toSort) {
  //...
}
```
Here the `entends` keyword is used to specify that the geric type `T` is must satisfy the "bounds" of Implementing `Comparable<T>`.
The same function in Rust would look like this:
```rust ,skt-default
pub fn sort<T : Ord>(to_sort : &mut Vec<T>) {
  //...
}
```
The `:` takes the place of "extends". `Ord` is a trait that allows items to be ordered similar to `Comparable` in Java.

You may be wondering, "Why doesn't Ord take a generic parameter?". After all `Comparable` in Java needs one because the interface is not specific to one implementation, and needs to refer to the same type. This is due to another feature, the "Self" type. 

If you'll recall, when we define a method on an object we write:
```rust ,skt-default
struct Foo;
impl Foo {
  fn bar(&self) {
    //...
  }
}
```
Where the 'self' variable is a way to pass the equivlent of "this" in Java. Well that's actually just shorthand for:
```rust ,skt-default
struct Foo;
impl Foo {
  fn bar(self : &Self) {
    //...
  }
}
```
Where `Self` is the type of the object. This isn't useful on an impl because you know the type, but on an interface it can be. For example you can define something like this:
```rust ,skt-main
trait Doubleable {
  fn double(self) -> Self;
}
impl Doubleable for i32 {
  fn double(self) -> Self {
    self * 2
  }
}
//then this method can be invoked
assert_eq!(4, 2.double());
```
So the `Self` type allows the trait to refer to whatever is implementing the interface without actually knowing what that is or having to have a generic itself.

## Features unique to Rust

Rust also has a few other features that help avoid so called "generics hell" that sometimes crops up in Java. 


  * Rust avoids generics hell
    * Ownership and borrowing save the day
    * https://doc.rust-lang.org/nomicon/subtyping.html 

### Where clauses
A simple one is "where" clauses. Suppose you had an interface like this:
```java
public <T extends Comparable<T>, ProcT extends Serializable, Processor<T>> byte[] serializeMaximumValue(List<T>, ProcT processor);
```
You could just do the same thing in Rust:
```rust ,skt-default
# //Normally Serialize is imported as: use serde::Serialize;
# pub trait Serialize {}
pub fn serialize_max_value<T : Ord, ProcT : Proc<T> + Serialize>(items : Vec<T>, processor : ProcT) -> Vec<u8> {
  //...
# vec![]
}
# pub trait Proc<T> {
#  fn do_something(item : T);
# }
```
The problem with this in both Rust and java is the generics really get in the way and make the function signature hard to read. So it's easier to use the `where` keyword to move these bounds over to the right, so they can be wrapped onto the following lines:
```rust ,skt-default
# //Normally Serialize is imported as: use serde::Serialize;
# pub trait Serialize {}
pub fn serialize_max_value<T, ProcT>(items : Vec<T>, processor : ProcT) -> Vec<u8> where
 T : Ord,
 ProcT : Proc<T> + Serialize {
  //...
# vec![]
}
# pub trait Proc<T> {
#  fn do_something(item : T);
# }
```
This is strictly equivlent, but hopefully easier to read.

### Impl keyword


In the case of a single trait/interface. In Java you might write __ where 'T extends’ is used to indicate there is some type T but the function doesn't care about the exact type of the object, provided it implements the specified interface. In Rust there is a shorthand for this using the 'impl’ keyword. __. This is most commonly used for return values for example __ specifies that this function is returning a _ but without providing the exact type. This is especially common when a function is returning a function. (Like a callback for example) __ here the function is specifying that it is returning a function with a particular signature but without specifying exactly which function it is returning. So you can think of impl as meaning “some implementation of this trait”. (It doesn't matter which) ‘impl’ only can refer to one trait. So if you need to use multiple, you'll have to use a where clause _ or an alias _.




<table width="100%">
<tr>
<td>

![Optimizer](images/professor.png)
</td>
<td width="80%">

>  *I can actually make methods invoked on an ‘impl’ parameter a lot faster. So take advantage of that.*
></td>
</tr>
</table>







### Associated Types

Finally there is one feature that cuts down on the burdon of generics a lot: "Associated types". In Java the Supplier interface is defined as:
```java
interface Supplier<T> {
  public T get();
}
```
To do the same thing in Rust, you could write:
```rust ,skt-default
trait Supplier<T> {
  fn get(&self) -> T;
}
```
Or you could instead write:
```rust ,skt-default
trait Supplier {
  type Item;
  fn get(&self) -> Self::Item;
}
```
Here `type` is a keyword, and `Item` is the name of the generic type rather than `T`.
This has a few advantages. First it allows the generics to be specified by name, rather than relying on order: 
```rust ,skt-default
# trait Supplier {
# type Item;
# fn get(&self) -> Self::Item;
# }
struct StringWrapper(String);
impl Supplier for StringWrapper {
  type Item = String;

  fn get(&self) -> String {
    self.0.clone()
  }
}
```
This can segnifigantly improve readability when there are multiple types. It also allows the type to be refered to without specifying the type. For example:
```rust ,skt-default
trait Countable {
    fn count(self) -> i32;
}
impl<T> Countable for T where T:Iterator {
    fn count(self) -> i32 {
        let mut count = 0;
        for value in self {
            count += 1;
        }
        count
    }
}
```
Here `Iterator` is refered to without needing to specify the generic associated with Iterator. This means that Countable does not itself have to be generic even though it operates on things that are. In the above code, they type of `value` is `T::Item`. A similar example using the supplier interface defined above would look like:
```rust ,skt-main
trait Supplier {
  type Item;
  fn get(&self) -> Self::Item;
}
//...
fn vec_of_value<S:Supplier>(supplier : &S) -> Vec<S::Item> {
  vec![supplier.get()]
}
```

In general the major difference between associated types and generics, is that for an associated type each implementation is tied to a speffic type. (For the language geeks this is called "unicity".) For example If you were to define a concrete class `LinkedList` it should use generics because any sort of item should be able to go into one. However when defining a more general trait for example `Iterator` in that case it should use an associated type, because each implementation of iterator is only going to have one perticular type associated with it. Usually this is the type of whatever collection is implementing it, which of course itself is likely a generic. For example:
```rust ,skt-default
trait Storage {
  type Item;
  //...
  fn insert(&self, item: Self::Item);
}
struct Message {/*...*/}
struct MessageStore {
  //...
}
impl Storage for MessageStore {
    type Item = Message;
    fn insert(&self, item: Message) {
      //...
    }
}
```
Here we have a `MessageStore` is tied to the type `Message` so it uses an associated type.



