# Flowcontrol
Similar to Java in Rust variables are defined to exist in the scope where they are declared. So you can do this
```rust ,skt-main
let mut i = 0;
while i < 10 {
    println!("Counter: {} ", i);
    i += 1;
}
```
but not this
```rust ,ignore
for i in 1..10 {
    let x = i;
}
println!("You can't do this: {}", x);
```
(SafetyMonitor: This is a bug, don’t do that.)  

Rust has `return` keyword, which works exactly like java:
```java
int square(int x) {
    return x * x;
}
```
```rust ,skt-default
fn square(x: i32) -> i32 {
    return x * x;
}
```
However in Rust expressions will return the last value in them if it is not followed by a `;` so you can just write:
```rust ,skt-default
fn square(x: i32) -> i32 {
    x * x
}
```

This also holds true for things like if statements
```rust ,skt-main
# fn something() -> bool {true};
let x = if something() { 1 } else { 0 };
```
is equivalent to
```java
int x;
if (something()) {
    x = 1;
} else {
    x = 0;
}
```
or
```java
int x = something() ? 1 : 0;
```
in Java.

Note that the parenthesis around an `if` or `while` conditional are totally optional. Because in Rust the conditional is just an expression in Rust, just like any other.

Below are examples of If, for, and while loops compared to their java equivalents. __ and __.

|Rust                  	|Java	|
|-----------------------|-------|
|`if _ { _ } else { _ }`|`if (_) { _ } else { _ }`|
|`match _ { _ => _ }`|`switch (_) { case _ : _ break; }`|
|`while _ { _ }`|`while (_) { _ }`|
|`loop { _ }`|`while (true) { _ }`|
|`for i in _ { _ }`|`for (X i : _) { _ }`|
|`for i in 0..n { _ }`|`for (int i = 0; i < n; i++) { _ }`|
|`break` / `break 'label` 	|`break` / `break label` |
|`continue` / `continue 'label`|`continue` / `continue label`|
|`return _;`|`return _;`|

There are a few differences: Rust does not have `do/while` loops. In Rust the `match` statement can use pattern matching and the cases do not "fallthrough" like `switch` in Java, so there is no need for a `break`.

The Rust `for` loop works like java’s “new” for loop, and doesn’t have the C style 3 part condition version.
```java
for (String item : collection) {
    //...
}

```
```rust ,skt-main
# let collection = vec![1,2,3];
for item in collection { //Item's type is infered
    //...
}
```
If you want to write a pattern like that, you can use a “range” operator instead.
```rust ,skt-main
for i in 0..10 {
    //...
}
```
The ".." operator produces an iterator between the value on the left and the one on the right (inclusive and exclusive). Rust also has a “loop” loop which is equivalent to `while (true)` in Java. The keywords `break` and `continue` work the same way they do in Java.
```java
while(true) {
    //...
    break;
}
```
```rust ,skt-main
loop {
    //...
    break;
}
```

Notice that the if, while, and for conditional arguments are just normal expressions that return a boolean. (As opposed to Java where it is a special construct surrounded by parenthesis). And because anything surrounded by curly braces is an expression which returns the last expression inside of it, it is possible to write a loop like conditional like this:
```rust ,skt-main
let mut i = 1;
while { i = i*2; i < 65536 } {
    //...
}
```
here the code is iterating over the powers of 2, but the code to update the variable `i` has been put in the top as part of the loop conditional. (The purpose of showing this is not to encourage doing this all over the place, but rather to show how the compiler sees things)

## Functions as arguments

In Java sometimes you pass functions as arguments. With a named function this is doen like this:

```java
Stream printableElements = stream.filter(Element::shouldPrint);
```

The Rust equivalent would be do this:
```rust ,skt-main
# struct Element();
# impl Element {
#    fn should_print(&self) -> bool { true }
# }
# let stream = vec![Element()].into_iter();
let printable_elements = stream.filter(Element::should_print);
```
Which usses the same `::` syntax.


Java can also create an “Anonymous function” (So called because it don’t have a function name) or “Closure” (so called because they can refer to local variables in the surrounding function) like this:

```java
Stream printableElements = stream.filter(e -> e.shouldPrint());
// or
Stream printableElements = stream.filter((e) -> {
    e.shouldPrint();
});
```
In Rust the syntax is:
```rust ,skt-main
# struct Element();
# impl Element {
#    fn should_print(&self) -> bool { true }
# }
# let stream = vec![Element()].into_iter();
let printable_elements = stream.filter(|e| e.should_print());
// or
# let stream = vec![Element()].into_iter();
let printable_elements = stream.filter(|e| {
    e.should_print()
});
```
Here the `|` characters replace the parentheses and the curly braces are optional if there is only a single statement.
