# Enums, tuples, and aliases

  * Range operator. Range on struct to destructure.
  * Enums in Java are way better than constants in other languages like C or Go. - You know they won’t get misassigned as numbers and you can use switch statements
  * Enums give more meaningful names to concepts without overhead, so you can name possible outcomes to improve API quality and reduce the chances of Error.
  * Enums can implement traits
  * bitflags

## aliases
Rust supports the concept of 'Aliasing’ this just allows you to refer to one type by a different name. This can be useful to add clearity for example:
```rust 
type Time = i64;
```
This will allow an i64 to be referred to as ‘Time’ like so:
```rust
# struct Message{timestamp:i64};
# let message = Message{timestamp:0};
type Time = i64;
let timestamp : Time = message.timestamp;
```
Note that Time is not a separate type. So a i64 can be passed to the function. This will not allow adding any special methods to the type, but it also means all the methods of the existing type are there. Because of these limitations, aliasing is generally not used for public interfaces, but to improve readability of code. It does have another important use: renaming an imported type. In Java if you have two types with the same name you have to refer to one by its fully qualified name. 
```java
import java.awt.List;
//...
class Example {
  private List awtList;
  //..
  void method () {
    //This is awkward...
    java.util.List<String> items = Arrays.asList(awtList.getItems());
  }
}
```
In Rust you could just rename one of types.
```rust,ignore
use some::value as other_value;
```

## Enums

Enums in Rust are different from enums in Java. They aren't just constants, they can have fields and multiple instances can be instantiated. Additionally different values can have different member variables. So in Java you can do __ but in Rust you can do __. Notice the _ parameter on the _. This can be different for different instances.

This is useful as it allows you to enumerate different possible return values. For example __. This would require a very awkward construction in Java.

## Option vs null
  * Option implements traits
    * Take method

In Java and variable can be assigned to null. (In fact there is no `null` in Rust) Rust uses an explicit Option type to convey when an object can be null. This was added in later versions of Java but it's not as helpful as it could be because it doesn't actually prevent assigning things to null, it just documents times when you expect that they can be null. As such it doesn't catch any NullPointerExceptions at compile time because these occur precisely when you don't expect things to be null. You might think the lack of nulls would result is very verbose code. But it's not much of a problem in Rust. Compare this __ to the equivalent Java __. The real advantage of making option explicit is not that it prevents null pointers but that it makes interfaces better. Take a look at Java's iterators. They need to have a hasNext() method because it cannot use null to differentiate “there are no more values” from a value that is null. Anyone who has implemented their own iterator will immediately recognize how annoying this can be.

Rust also supports `destructuring`. This allows assignment statements to extract values from structures. For example you could write __ to split apart a structure. Here the struct is being split up and assigned to individual variables. There is an even fancier version of this where it can be used in a conditional. Here is an example of “if let” this is extracting and assigning the value ‘x’ but does so only if the surrounding pattern matches. So in this case if the option is ‘None’ this will not match, the condition will be false and the ‘if’ will execute the code in the else block. You can do the same thing with a “while let”. __ As you can see this is useful when there is a 

## Tuples

Rust supports Tupples. Tupples are just like 'Pair’ in `apache lang` in Java or Map.Entry. They are a simple wrapper of multiple objects. The syntax looks like:
```rust
let pair : (i32, String) = (5, "Hello".to_string()); 
```
which is a tuple of a integer and a String. Tupples aren't full blown classes so the fields don't even have names. They are accessed by number. 
```rust
let pair = (10, 20);
let ten = pair.0;
let twenty = pair.1;
//Tupples can also be named
struct Position(f64, f64);
let pos = Position(1.4, 3.2);
```
Tupples should only be used when it is very clear what the represent, because without accessor methods or named fields their meaning can be ambiguous to someone who doesn't fully understand the context.

Tupples can have 0 or more values. An empty tuple is just `()` this litterly contains nothing and has no methods. An empty tuple is actually the return type of a “void” method. (This is sometimes called “the unit type”) 

Tuples with two parameters are the most common. IE . But 3 or more is possible. 
```rust 
let tripple = (1, 1.0, "one");
let quad = (2, 2.0, "two", true);
```
A single element tuple might seem pointless but it does provide a way to name something. IE: 
```rust
struct Distance(f64);

let london_to_NY = Distance(3459.3,);
```
Which can sometimes help with clearity. (Note: the single element tuple must have the trailing ‘,’ to distinguish it from a normal expression in parenthsis.
  * https://medium.com/@robertgrosse/ref-patterns-destructuring-and-invisible-borrows-2f8ae6902656

## Match
  * Match guards / binding

In the above example we introduced a new construct `match`. Match works like a souped up version of Java’s switch statement. It differs in a few important ways: It must be exhaustive, There is no fallthrough (and hence no break), it is an expression so it can return a value, It has pattern matching which lets you define what to match against, it can capture variables so the right hand side can use them. Here is a match statement that shows off all of these __ compared to the equivalent Java __. 

Match works with destructuring __. It works on Options and Enums __. If all cases are covered, there does not need to be a default branch. It also works with ranges __. (The ‘_’ means “you figure it out” and is a default branch.) If the paths are not mutually exclusive for example __. The first one that applies will be taken. IE __.

Sometimes in a destructuring expression you will see the word 'ref’. This is equivalent to a ‘&’ in the corresponding place on the other side. IE __ is equivalent to __.

