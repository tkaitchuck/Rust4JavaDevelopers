# Variable declaration
Below is an example of a variable being declared in Java and the equivalent expression in rust.
```rust
let foo : i32 = 5;
```
```java
int foo = 5;
```
The keyword ‘let’ is used to indicate you are declaring a variable. This is followed by the variable name, then a colon and then the type. 
The reason Rust puts the type one the right is because the type is optional anytime it can be inferred by the compiler. Which means that most of the time we can just write:
```rust
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

Rust also supports unsigned integers these cannot be negative and use their highest order bit to represent higher numbers just like any other bit. The range of values that can be represented in each is listed in the table below. 

| Type | Min | Max   |
|------|-----|-------|
| u8   | 0   |2^8-1  |
| u16  | 0   |2^16-1 |
| u32  | 0   |2^32-1 |
| u64  | 0   |2^64-1 |
| u128 | 0   |2^128-1|

Constants can also explicitly specify their type. (Similar to Java) The examples below are equivalent.
```rust
const secondsPerMinute : i64 = 60;
```
```java
static final long secondsPerMinute = 60;
```

By default all variables in Rust are final. So instead of a final keyword there is the `mut` keyword to indicate the variable is mutable (non-final). For example:
```rust
let mut counter = 0;
``` 
The advantage of using `mut` rather than `final` (besides being fewer characters) is it makes the declaration is far more likely to be accurate. In Java it very common to simply omit the final keyword even when a variable is final. As a result when reading Java you often don’t know if a variable is final or not. 

Safety monitor: Rust's default declaration actually provides an even stronger guarantee than Java's final, because we not only enforce that the variable won't be reassigned, but also that it's contents won't be changed.

A ‘final-by-default’ policy makes reasoning about code a lot easier, because you know if something is going to change. This makes writing multi-threaded code easier, because if something is immutable can always be shared safely between threads. (We’ll come back to this in a later chapter).

## Vec
Rust has a built in type called `vec` which is short for "Vector". It is equivlent to Java's `ArrayList`. However because Rust does not have "primitive wrapper" types like Java's "Integer" and "Byte" (as opposed to "int" and "byte") and is capable of putting primitives in collections directly without any overhead `Vec` receives a lot more use in Rust than `ArrayList` does in Java. It is often used in places where in Java you would use an array or a generic 'Collection'. 

### String
