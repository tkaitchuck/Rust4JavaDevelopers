# Method Signatures
Below is a Java method signature and the equivalent rust signature. 
```java
void printNum(int num);
```
```rust ,skt-default
# trait Example {
fn print_num(num: i32);
# }
```
Things to notice: 
The Rust method starts with the keyword “fn” this simply indicates that a function is being declared. This is followed by the method name and each of the parameters to the method in parentheses. Unlike Java where the type appears on the left followed by the argument name, in Rust the name comes first followed by a ':’ followed by the type. (Similar to how variables are declared)

If the function had a return value it would be declared on the right hand side like this:
```java
int add(int a, int b);
```
```rust ,skt-default
# trait Example {
fn add(a: i32, b: i32) -> i32;
# }
```
There is no “void” in Rust. If there isn't a return value, just don't provide one.

Just as in Java the method body is in between the curly braces.
```java
int add(int a, int b) {
    return a + b;
}
```
```rust ,skt-default
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```
## Borrowing
Besides being mutable or not, in Rust a variable can also be “borrowed”. When the parameter to a method is borrowed, it means the method promises not keep the parameter after the method has returned. Or anything it obtained from the parameter. 

A method that takes a borrowed parameter cannot assign the parameter to a member variable. Also it can only pass that parameter to other methods which also borrow it. (Otherwise it would be able violate its contract indirectly.)

<table width="100%">
<tr>
<td> 

![Safety monitor](images/borrow.png)
</td>
<td width="80%">

>  *I will act as the borrow checker to make sure you don't make a mistake.*
</td>
</tr>
</table>


To indicate a parameter is borrowed, in the method signature, place a “&” in front of the type.

As an example, in Java you might define a method 'isSorted’ like this 
```java
boolean isSorted(List<Integer> values);
```
that returns a boolean indicating if a list is sorted. In Rust you would add an ‘&’ in front of the type in the method signature to indicate that it will not retain any references to the list or it's contents when the method returns.
```rust ,skt-default
# trait Example {
fn is_sorted(values: & Vec<i32>) -> bool;
# }
```
It is helpful to think of 'borrowed’ as being part of the type. IE “The method takes a borrowed list.” 

This declaration provides a strong and useful guarantee to the callers of a method. But it would be worthless if in a newer version, the method just deleted the ‘&’ from it's signature and removed the guarantee. To prevent this, and to make the guarantee explicit in the caller's code, when a method is being invoked; When a method that borrows a parameter is invoked, the caller puts an ‘&’ in front of the variable name being passed in. For example 
```rust ,skt-main
# fn is_sorted(values: & Vec<i32>) -> bool { true }
# let values = vec![1, 2, 3];
if (is_sorted(&values)) {
    //...
}
```
This is only needed if the variable being passed isn't itself borrowed. (Otherwise it would be redundant, because it can only pass it to methods which borrow it.)

Passing a parameter to a method that borrows is sometimes referred to a the parameter being “lent” to the method. 

### Mutability

Similar to variable declaration, if you want to modify a borrowed parameter, you use the 'mut’ keyword. This goes right in front of the type.
```java
void populateCounts(HashMap<String, int> itemCounts);
```
```rust ,skt-default
# use std::collections::HashMap;
# trait Example {
fn populate_counts(item_counts: &mut HashMap<String, i32>);
# }
```
`mut` can be thought of a part of the type. IE “a borrowed mutable Hashmap” as opposed to “a Hashmap".

When 'Borrowed’ is combined with 'mut’ the ‘&’ goes first. If you wanted to write a method to sort a list, it would take a borrowed mutable list.
```rust ,skt-default
# trait Example { 
fn sort(names: &mut Vec<String>);
# }
```
This signature means `sort()` may change the list, but only during the invocation of the method and will retain no references to the list or its contents once the method returns.

## Javadocs
Documentation is an area where Rust and Java are very similar. In Java you might add a javdoc like this 
```java
/**
 * Computes an approximation of {@code 1/sqrt(a)} segnifigantly faster. 
 * However compared to using {@link java.lang.Math.sqrt()} the result is much less accurate.
 * @param   a   the value to compute the inverse square root of.
 * @return an approximate inverse square root of the passed parameter.
 */
public double fastInvSqrt(double a);
```
which can be automatically translated into HTML documentation. 

## Rustdocs
Rust has rustdocs which work similarly. You could write the following:
```rust ,ignore
/**
 * Computes an approximation of `1/a.sqrt()` segnifigantly faster.
 * However compared to using [`sqrt`] the result is much less accurate.
 * # Examples
 * ```
 * let a = 7.0_f64;
 * let exact = 1.0 / a.sqrt();
 * let approx = a.fast_inv_sqrt();
 * assert!((approx-exact).abs() < 1e-5);
 * ```
 */
```
Or instead of “/**” and a block comment, you can use “///” and line comments. So the following is equivalent:
```rust ,ignore
/// Computes an approximation of `1/a.sqrt()` segnifigantly faster.
/// However compared to using [`sqrt`] the result is much less accurate.
/// # Examples
/// ```
/// let a = 7.0_f64;
/// let exact = 1.0 / a.sqrt();
/// let approx = a.fast_inv_sqrt();
/// assert!((approx-exact).abs() < 1e-5);
/// ```
```
(This helps with small comments as it doesn't require an extra line at the top and bottom.)

Javadocs have a number of common tags such as “@param” and “{@link }”. Below are some common ones and their Rust equivalents.

| Type         | Java           | Rust        |
|--------------|----------------|-------------|
|Link to method|``{@link Foo#bar() }``|``[`Foo::bar()`]``|
|Link to URL   |```<a href="https://google.com"> google</a>```|```[google](https://google.com)```|
|Code snippit  |``{@code foo.bar()}``|`` `foo.bar()` ``<br/>or<br/>```` ``` ````<br/>&nbsp;&nbsp;``foo.bar();``<br/>```` ``` ````|
|Parameter     |``{@param foo bla bla}``| N/A Documented through examples and code snippits|
|Return        |``@return bla bla``| N/A Documented through examples and code snippits|
|Examples      |``<pre>`` <br/> ``{@code ``  <br/>&nbsp;&nbsp;`//...` <br/> `}` <br/> ``</pre>``| ``# Examples`` <br/> ```` ``` ```` <br/>&nbsp;&nbsp;`//...`<br/> ```` ``` ````|
|See also      |``{@see  "Bla bla"}``|``# See Also`` <br/> `Bla bla`|
|Custom tag    | Requires custom javac args | ``# My custom section`` <br/> `//...`|
|Bulleted list |`<ul> `<br/>`<li>one</li> `<br/>` <ul> <li>one point one</li> </ul> `<br/>` </ul>`|` * one `<br/>` ** one point one `|


Difference between Javadocs and Rustdocs is that to do formatting, in Java you would inject HTML tags, where as in Rust you use Markdown syntax. So the following Javadoc and Rustdoc are equivalent __ vs __. As you can see this improves readability a lot.

To generate an view your docs you can run _. Which will put the documentation in _. When you publish your code to Crates.io (Rust’s main package repo, the docs will be published automatically.) There will be more on Crates.io and the Cargo command in chapter _.

Additional information for the doc can also be placed inside of the method, if for some reason that makes more sense from an organization point of view. So the following are equivalent __ and __.

Docs can also be hidden (If for example a feature is still being tested) like this __. Similarly you can use the _ annotation to mark them as being platform specific. For example __.

It is common practice in Rust to write a small example for how to use each function rather than documenting all of the input and output parameters like you would in Java. So instead of __ you might write __ in Rust. These examples aren’t just for show, they also get automatically turned into unit tests. For example __. This makes sure your documentation stays up to date with the code. If you want to hide a few lines of setup at the top of an example you can use _. For example __. 

## Macros
Ok, let’s finally get around to writing HelloWorld:
```rust
/// Prints "Hello world!".
fn main() {
    println!("Hello World!");
}
```
You may be wondering: “What is that exclamation point doing at the end of that function name?” The answer is 'println!’ is not a normal function it is what is called a “Macro”. Rust macros are "hygienic" so they don't have the horrifyingly dangerous properties of C and C++ macros, and are designed to be safe.

You can think of macros as a function that does things that functions can't normally do. In this case ‘println’ supports string templates. 
```rust
fn main() {
    let name = "Ferris";
    println!("Hello {}!", name);
}
```
The template is validated at compile time. So if you were to type 
```rust, ignore
println!("Hello {}!");
```
or 
```rust, ignore
# let name = "a"; let name = "b"
println!("Hello {}!", name1, name2);
```
the mistake would actually be a compile error. (BorrowChecker: Which is great because it means you can't write incorrect code. Optimizer: and has the added bonus of not spending any CPU at runtime parsing and verifying the template.) This kind of verification is something a normal function couldn't possibly do. Will get into how this implemented in a later chapter. But for now you should think of an elimination mark, as an alert that there's something unusual about that function and you should read its documentation.

Another place you'll see macros, is for initilizing collections or places where you might find "varargs" in Java. For example you can initilaize a Vec with the `vec!` macro:
```rust ,skt-main
let number = vec![1, 2, 3, 4, 5];
```