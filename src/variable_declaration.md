# Variable declaration
Below is an example of a variable being declared in Java and the equivalent expression in rust.
```rust ,skt-main
let foo : i32 = 5;
```
```java
int foo = 5;
```
The keyword `let`is used to indicate you are declaring a variable. This is followed by the variable name, then a colon and then the type. 
The reason Rust puts the type on the right is because the type is optional anytime it can be inferred by the compiler. Which means that most of the time we can just write:

```rust ,skt-main
let foo = 5;
```
The equals sign for assignment works the same way it does in Java. Primitives also work the same way but they have different names. Below is a table to help:

| Java  | Rust  |
|-------|-------|
| long  | i64   |
| int   | i32   |
| short | i16   |
| byte  | i8    |
| double| f64   |
| float | f32   |

Rust also supports unsigned integers. These cannot be negative and use their highest order bit to represent higher numbers just like any other bit. The range of values that can be represented in each is listed in the table below. 

| Type | Min | Max   |
|------|-----|-------|
| u8   | 0   |2^8-1  |
| u16  | 0   |2^16-1 |
| u32  | 0   |2^32-1 |
| u64  | 0   |2^64-1 |
| u128 | 0   |2^128-1|

Constants can also explicitly specify their type. (Similar to Java) The examples below are equivalent.
```rust ,skt-main
const SECONDS_PER_MINUTE : i64 = 60;
```
```java
static final long SECONDS_PER_MINUTE = 60;
```

By default all variables in Rust are final. So instead of a final keyword there is the `mut` keyword to indicate the variable is mutable (non-final). For example:
```rust ,skt-main
let mut counter = 0;
```
The advantage of using `mut` rather than `final` (besides being fewer characters) is it makes the declaration is far more likely to be accurate. In Java, it is very common to simply omit the final keyword even when a variable is final. As a result, when reading Java, you often don’t know if a variable is final or not. 


<table width="100%">
<tr>
<td> 

![Safety monitor](images/borrow.png)
</td>
<td width="80%">

> *Rust's default declaration actually provides an even stronger guarantee than Java's final, because we not only enforce that the variable won't be reassigned, but also that it's contents won't be changed.*
</td>
</tr>
</table>

A ‘final-by-default’ policy makes reasoning about code a lot easier, because you know if something is going to change. This makes writing multi-threaded code easier, because if something is immutable can always be shared safely between threads. (We’ll come back to this in a later chapter).

# Common types

There are some common builtin types that appear in a lot of the examples. The details of each will be explained later.

## Box
The first and most embarrassingly simple of those is `Box`. Box is a collection of exactly one item. (It can't even be empty) It's just a wrapper around a generic value. If you were to implement Box in Java it would look like this:
```Java
class Box<T> implements Iterable<T> {
  private final T item;

  public Box(T item) {
    this.item = item;
  }

  public Item getItem() {
    return item;
  }

  @Override
  public Iterator<Item> iterator() {
    //...
  }
}
```
You may be wondering: “Why would I ever need such a useless class?". Usually you don't. But it comes up as a work-around to some restrictions. 
These cases will be explained when they come up.

Declaring a box is done like this:
```rust ,skt-main
let boxed = Box::new(1);
// or if you want to be explicet about the type
let boxed2 : Box<i32> = Box::new(2);
```
The `::` operator in the example above actually does exactly the same thing as it does in Java. It is how you can refer to a static method without invoking it (i.e. `Arrays.sort(stringArray, String::compareToIgnoreCase);`). But in Rust, once you have a method reference, instead of calling `method.run()` or `method.call(arg)` you can just invoke it: `method()` or `method(arg)`. So `Box::new` refers to the constructor of `Box` just like it would in Java, and the `(1)` following it invokes the method passing the argument `1`.

## Vec
Rust has a built in type called `Vec` which is short for “Vector". It is equivalent to Java's `ArrayList`. However because Rust does not have "primitive wrapper" types like Java's `Integer` and `Byte` (as opposed to `int` and `byte`), and it is capable of putting primitives in collections directly without any overhead. `Vec` receives a lot more use in Rust than `ArrayList` does in Java. It is often used in places where in Java you would use an array or `Collection`.

The syntax looks like this:
```rust ,skt-main
# fn get_numbers() -> Vec<i32> { vec![] }
let mut numbers : Vec<i32> = get_numbers();
numbers.push(5); //Pushes 5 onto the vector. (vec will automatically resize if needed)
```
Because adding a layer of abstraction does not add any overhead in Rust, it is common to pass a `Vec<u8>` where you would use a byte array in Java.

## String
Rust has a string type. It's a little different from Java's. While Java's String is built out of a `char[]` where each `char` is a UTF-16 code point, Rust's String is based on a `Vec<u8>` where the bytes are valid UTF-8.

This has advantages for size and serialization. But the real gain is that Strings can be mutable if desired. If declared with `let mut directions : String = ...`, it is mutable, but if declared with `let name : String = ...`, it is not. 

Of course, passing around mutable Strings would be error-prone in Java. We'll cover why this is not a problem in Rust in the [chapter on Ownership](./ownership.html).
