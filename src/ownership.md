# Memory safety and GC
Like Java, Rust has automatic memory management so you never have to worry about explicitly allocating or deallocating objects. However unlike Java it does not have a garbage collector. This may sound like a contradiction but it's not. 

It turns out, not coincidentally, the same static analysis that solves the problem of allowing sharing or mutability but never both, also solves the problem of perfect garbage collection. 

One way to think about it is to think of Rust as having compile time garbage collection. The compiler works out where in your code objects are no longer used automatically generates the necessary code to deallocate them.

So like Java this means Rust does not suffer from memory leaks, use-after-free bugs, dangling pointer bugs, or buffer overflows that plague most compiled languages. At the same time Rust does not have the overhead of garbage collection or the associated runtime which has prevented languages like Java and C# from reaching the performance of C++ in “object heavy” applications.

# Ownership

To explain how Rust achieves automatic memory management without garbage collection and a number of other more advanced features we need to first explain Ownership. 

Ownership is a common pattern in programming in any language even though it really isn’t a part of most languages themselves. It’s best to see this through example. In Java compare the method `addAll` on `TreeSet` to the static method `Collections.sort`. Both provide ways to sort a list of items by passing it into a method, but the contract of these methods is quite different. In the case of the `TreeSet.add` ownership of the values is being implicitly transferred to the TreeSet. The caller should not hold onto references to the items added to the Set. If they want to modify one of the items they should first remove it from the set. Otherwise the change could affect the of sort order and break the TreeSet resulting in undefined behavior. On the other hand `Collections.sort` does not have this restriction. In fact it is implicit in the contract of the method that the implementation won't hold onto references to the items passed after the method returns. If the implementation of sort were to modify the the list after it returned it would surely break the caller's code. `Collections.max` is similar but it has the additional implicit assumption that it won't modify the passed list. 

In Rust these assumptions are explicitly declared as part of the method signature. So if in Java we were to declare integer speffic versions on each of these, it would look like this: 
```java
public interface IntegerSet {
  //...
  public void addAll(List<Integer> list);
}
public static Integer max(List<Integer> list) {
   // ...
}
public static void sort(List<Integer> list) {
   // ...
}
```
in Rust an equivlent method are declared as:
```rust ,skt-default
pub trait MySet {
  //...
  fn add_all(list: Vec<i32>);
}
fn max(list: &Vec<i32>) -> Option<i32> { 
  //...
# list.iter().cloned().max()
}
fn sort(list: &mut Vec<i32>) {
   //...
#   list.sort();
}
```
Notice that the type of the argument changed. When it is just `Vec<i32>` the method is taking ownership of the value. 
But when it is `&Vec<i32>` it is a borrowed vector, meaning the caller still retains ownership. And a `&mut Vec<i32>`
is a borrowed mutable vector meaning that even though the function is not taking ownership, it may modify the provided vector.

For any given object there is one owner. When that variable is reassigned or goes out of scope the value is dropped. 
This applies transitively. For example in the function:
```rust ,skt-default
fn process(items: Vec<String>) {
  //...
}
```
when the function returns, the vector `items` and all of the strings in it will be dropped from memory.

This might seems like it doesn’t allow cycles. There are ways to create cycles, but for the most part they aren’t needed.
Why this is the case will be covered later in [“How Rust makes you a better Java programmer”](./rust_makes_you_better_at_java.html).

The main tool that used in conjunction with ownership is borrowing.

# Borrows
  * Output to be populated
  * Sort example
  * Also called reference (Not the same as a C++ ref, more like a smart pointer)
  * Primitive (copy by value) vs pointer
  * Copy vs move (is similar)

In addition to compile time memory management and [guaranteed thread safety](./concurrency.html)),
explicit ownership opens up a lot of useful patterns.

In Java a common pattern is to pass around a byte array with a offset and length to provide access to a part of an array without making a new copy. For example see <code class="java">java.io.OutputStream.write(byte[] b, int off, int len)</code> or <code class="java"> java.io.FileInputStream.read(byte[] b, int off, int len)</code>. In Rust you can use slices. So you can write 
```rust ,skt-main
# use std::io;
# use std::io::prelude::*;
# let mut output = io::stdout();
let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
output.write(&buffer[2 .. 8]);
```
which passes a segment of a byte array to the `write` function. In addition to convince and performance, it allows you specify if the slice is being passed can be written to or just read from have this enforced by the compiler. So if you have a function that looks like this:
```rust ,skt-default
# pub struct MyConfig(i32);
pub fn apply_config(config : &MyConfig) {
  //...
}
```
you can be sure that the `config` object won’t be changed by passing it to the function. No defensive copies required. Similarly, above the caller is guaranteed that `output.write(&buffer)` won’t modify the contents of the buffer.

Borrowing also works with loops. When using a for loop to iterate over a collection, you can either pass the for loop the collection itself 
```rust ,skt-main
let strings = vec!["foo", "bar", "baz", "bat"];
for value in strings {
  println!("Hello {}", value);
}
```
consuming the collection in the process, much a like a stream in Java. Or you can let it borrow the collection
```rust ,skt-main
let strings = vec!["foo", "bar", "baz", "bat"];
for &value in &strings {
  println!("Hello {}", value);
}
```
If the collection is borrowed then inside the loop each entry will itself be borrowed. This is actually a important feature. In Java a for-each loop can't take an iterator only an iterable. This is because if it did you could write
```java
Iterator<String> iter = //...
for (value : iter) {
  //...
}
for (oops : iter) {
  //Can't get here...
}
```
the second loop wouldn’t make any sense because it can’t get any data because there is no way to invoke .iterator() again and reread the data. For this reason Java introduced a second concept ‘streams’ which don’t work with for loops. Because there was no way to convey to the compiler that the for loop will consume the values it's iterating over, Java had to create two separate concepts `stream` and `iterator` that don’t work together. You can either write 
```java
List<Foo> foos = //...
for (Foo f : foos) {
  if (meetsCriteria(f)) {
    process(f);
  }
}
```
or you could write
```java
List<Foo> foos = //...
foos.stream().filter(f -> meetsCriteria(f)).forEach(f->process(f));

```
but you can’t mix these ways of coding. In Rust this can be conveyed by either passing or lending the collection to the for loop. This allows the concepts of stream and iterator to be unified into a single simple interface and work with for loops without the risk of accidentally reusing the consumed stream allowing you to write:
```rust ,skt-main
# struct Foo(i32);
# fn get_foos() -> Vec<Foo> { vec![Foo(1), Foo(2)] }
# fn meets_criteria(f : &Foo) -> bool { true }
# fn process(f : Foo) {}
let foos : Vec<Foo> = get_foos();
for f in foos.into_iter().filter(|f| meets_criteria(f)) {
  process(f);
}
```

In addition to these there are a bunch of other common patterns.
```rust ,skt-default
# trait Example {
fn read_from_buffer(&self, buffer : &[u8]);
# }
```
Here a method is borrowing a parameter but it's not modifying it. When is the messages returned your guaranteed it is not still holding onto it. 
```rust ,skt-default
use std::path::Path;
trait Config {
  //...
  fn get_storage_location(&self) -> &Path;
  //...
}
```
Here an accessor method is lending the caller some of the object's internal state (in a read only way) the calling code
cannot invoke any further methods on the object until it drops the reference to the data that was returned from this method. 
This is a great pattern for simple accessors that would not be safe in Java because they would be exposing the internal 
state of the class and potentially violating it's invariants. While it may not always be a good idea to expose internal 
representation, this provides a way to do it safely that does not violate the integrity of class, and still allows the 
implementation to change in the future. (It can always construct the returned object if needed)
```rust ,skt-default
use std::collections::HashMap;

trait Config {
  //...
  fn set_attributes(&mut self, attributes: HashMap<String, String>);
  //...
}
```
here the `set_attributes` function is making explicit that when called it is now the owner of the provided `attributes` 
and the caller no longer has any references to it. In Java would be dangerous. Usually to prevent this a defensive copy 
is made. However this comes at a performance cost. To avoid this sometimes Java programs just skip it because the transfer 
of ownership is understood and users know not to do this. For example when inserting an object into a HashSet, it is understood 
that you should not modify the object afterwards. But nothing actually prevents this. 

TODO:
  * Similar pattern getting an entry by key and doing .or_insert() += 1. 

The rules for ownership and borrowing are straight forward: __

...

There are more exotic ways to handle objects then in general aren't really needed the overwhelming majority of the time. 
These include RC (which allows ambiguous ownership where the item is dropped when all references go away.) 
ARC which is similar, but thread safe. This is generally used for Top-level classes with business logic that may need to 
be referenced from multiple places and live for a long time.


All of these compile time rules can be broken by declaring code ‘unsafe’ but you shouldn’t go around do that, 
because it will mean the compiler won’t be able to protect you. Instead the pattern in Rust is to use ‘unsafe’ to build a 
small generic primitive which is itself safe but is for reasons that the compiler doesn’t understand. Then depend on that
component where you need it. There are many such components publicly available, and we’ll cover some of them in depth 
in this book. A short list of common ones is below __
  * SplitAtMut
  * Cell
  * RefCell
  * Rc/Arc

  * Cell provides internal mutability. Ie you can change data when immutable, but only behind an interface.
    * Can't violate normal immutability rules, because cell requires ownership.
    * Can't violate normal borrowing rules either.
    * Cell forbids references to its contents.
    * Things containing cells are not allowed to cross thread boundaries
    * When you see realize the field is mutable, and can change between times you use it.
  * RefCell allows in stead or replacing a value to change it like cell the ability to borrow and mutable borrow the contents.
    *  like cell, can't use to violate normal parameter guarantees
    * Can't cross thread boundaries.
    * Still safe. Single writer of multiple readers enforced at runtime.


‘Cell’ is a class included in the standard library. It allows the value it is wrapping to be replaced. So you can write a struct like this __ and then modify that field it in a function that only has an immutable borrow like this __. This circumvents the normal mutability rules, and as such the compiler will not allow types using ‘Cells’ to cross thread boundaries. Similarly there a type ‘RefCell’ that allows the value to be modified (as opposed to replaced). __ Here a hashmap is being defined that can be updated by a function that only has a _. ‘RefCell’ does not actually abandon safety all together. While you can get a mutable reference out of an immutable object like so __ it actually just moves the safety check from compile time to runtime. So if your code actually does something bad, like attempts to get two mutable references at the same time, it will panic. <It is still correct if you have full unit test coverage>
  * Ref and RefMut are returned. These act as a lifetime tracker which allows the enforcement of one writer at a time.  

In general using ‘cell’ or ‘RefCell’ a lot is considered bad design. ‘Cell’ and ‘RefCell’ should be reserved for special cases that don’t impact the externally visible functionality. For example they provide a easy way to add things like counters, metrics, debugging information to an existing object without having to refactor all the code that is accessing it. Similarly they are frequently used when constructing Mock objects for test purposes. (The actual code may be accessing the object in a ‘read-only’ way, but the mock still may want to record what calls have occured) 

Reference cycles and ambiguous ownership is an anti-pattern in Java, and a really really aggressively discouraged anti-pattern in Rust. But one place it tends to get asked about a lot is doing GUI programing. For example when rendering a window with many widgets it is easy to jump to the conclusion that links are needed between all sorts of objects as changes to one may involve changes to another. This generally arises from the false belief that objects in an object oriented system should be directly modeled after real life objects. often this is not the case because concerns are cross-cutting. If you're writing code in Rust this pattern will fail faster. While this might be frustrating to the author it's actually good because it prevents you from writing bad code. There are actually a lot of good talks on this subject, such as _ECS at rustconf_ and _xi-GUI_. I'm not going to go into the details here because these problems are application specific. However if you find yourself fighting the borrow checker or tempted to use unsafe or RefCell all over the place, it’s probably worth taking a step back and re evaluating the broader design. …….. data oriented design.


# Returning borrowed values
Borrowing is not just for parameters, it is also for returned values. 
The simplest case is where the returned value is derived from an input parameter. 
For example:
```rust ,skt-default
# struct Foo { bar: Bar }
# struct Bar {}
fn get_bar(foo: &Foo) -> &Bar {
    &foo.bar
}
```

Another case is where the value being returned comes from 'self’ and is being lent to the caller. For example:

```rust ,skt-default
# struct Bar {}
struct Foo { 
    bar: Bar 
}
impl Foo {
    fn get_bar(&self) -> &Bar {
        &self.bar
    }
}
```

In both these cases the caller is bound by the contract of borrowing, exactly the same as though it were provided as an input parameter.
If it helps, imagine the rest of the function were factored out into a private method and had the result as a borrowed parameter. For example:

<table width="100%">
<tr>
<td>

```rust ,skt-default
# struct Foo { bar: Bar }
# struct Bar {}
impl Foo {
    fn get_bar(&self) -> &Bar {
       &self.bar
    }
}
fn process_foo(foo: &Foo) {
    let bar = foo.get_bar();
    process_bar(bar);
}
# fn process_bar(bar: &Bar) {}
```
</td>
<td>

This is equivalent to:
```rust ,skt-default
# struct Foo { bar: Bar }
# struct Bar {}
fn process_foo(foo: &Foo) {
    process_bar(&foo.bar);
}
# fn process_bar(bar: &Bar) {}
```
</td>
</tr>
</table>

When returning borrowed values it is occasionally ambiguous where the returned value came from. 99% of the time the compiler will work it out automatically. But sometimes there are cases that aren't so clear. 
For example here the compiler won't be able to work it out, because it only looks at one method at a time. 
```rust ,ignore
fn function_with_two_inputs(arg1: &Something, arg2: &SomethingElse) -> &Output {
    //...
}
```
In such a case you can label your inputs. Like so:

```rust ,ignore
# struct Something();
# struct SomethingElse();
# struct Output();
fn function_with_two_inputs(arg1: &'a Something, arg2: &'b SomethingElse) -> &'a Output {
    //...
}
```
(Usually 'a, 'b, 'c, etc are used.) And the apply the same label to the output. In this case it is indicating that the output is derived from the first input and not the second.
So it is explicit which input you intend the output to have come from. (The compiler will still check to make sure the labels are correct.)

This also happens to be the same syntax that you would use to label loops for the `break` and `continue` keywords, which work similar to Java:
```rust ,skt-default
# struct Item();
# fn is_bad(item: &Item) -> bool { true }
fn labeled_loops(collection: &Vec<Vec<Item>>) -> bool {
    'outer: for group in collection {
        'inner: for item in group {
            if (is_bad(item)) {
                break 'outer;     
            }
            //...
        }
        //...
    }
    return false;
}
```

In the above case the thing being labeled is the loop. In the first case the thing being labeled is the `lifetime` of the borrow.
It is also possible to have a generic lifetime. This is useful for cases where which parameter a returned value is derived from
is not known at compile time. For example:

```rust ,skt-default
///Returns the longer of two strings.
fn get_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

# Pass by value vs Pass by reference

Similar to Java, Rust's function calls are normally “pass by reference” meaning that if you pass an object to a function and modify it the caller will see those modifications. However also like Java, primitives are an exception to this and are “pass by value”. Meaning if you pass an integer to a function and it increments it. The caller will not see the change.

In Java this is just a hardcoded rule and only primitives are copied. In Rust you can define your own types that are treated this way by having them implement “Copy” which is what is called a “marker trait”. It is similar an interface with no methods in Java indicating something about a class. (Like ‘Cloneable’ or ‘Serializable’)

# TODO
  * String types (mutability, borrowing, assignment as move)
    * String concatenation
      * Format! In example
    * Stringbuilder
    * In Java Strings are Immutable, and a primitive. You might not think about it too much but this is an essential language feature. If Java hadn’t provided a single standard String implementation in the standard library, or if they had chosen to make it mutable it would be very difficult to work in the language. Imagine if every time you passed a string into a method you had to make a defensive copy or carefully check the method’s Javadocs to make sure it doesn’t modify the string. So it might seem surprising that Rust went into a different direction. However there is a very simple reason for this, Rust’s methods always declare if they need to modify the value being passed. __. Similarly the caller has to explicitly pass either a mutable or immutable reference or slice of the string. So there is no ambiguity. A function can never pass a string somewhere and have it unexpectedly modified. 
  * Fixed size arrays
  * Byte array and ByteBuffer
  * Vec and arraylist
    * Strings are actually Vecs of UTF-8 characters/bytes
  * Raw strings for multi line constants
  * Slices
    * Str is a slice of a String (get it?)
    * Should be after indexes and range traits are introduced (Operator overloading)
    * A slice of a vec is an array
      * Made possible by ownership
