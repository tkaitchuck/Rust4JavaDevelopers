# Operator Overloading

  * Size() length, get(), []

Traits are also used to what in other languages is called “operator overloading”. For each operator like ‘+’ for example there is an associated trait, which has a method that can be implemented. If an object has an implementation of the trait, you can use the operator as a shorthand for invoking that method. For example in Java it is very common to implement Compairable. However only primitive types can use the ‘<’ and ‘>’ operators. So you have to write:
```java
if (a.compareTo(b) < 0) {
    //...
}
```
Where as in rust if your type implements the `Ord` or `PartialOrd` traits you can just write:
```rust ,skt-main
# let a = 1; let b = 2;
if a < b {
    //...
}
```
Note that `Ord` compares the object against others of the same type so unlike Java you don't have to worry about a class cast exception at runtime.

Unlike C++ or Scala all of the operators that can be overloaded and the signature of their corresponding methods is pre-defined by the language. Because implementing a trait is only allowed in the package that defined either the trait or the type you can add support for operators to your own types, but you can't add or alter their behavior on someone else's types. So you can't add you own “pope operator” that looks like “<+|:-)”. And operators can't be used to do anything crazy or surprising.

Operators are just syntactic sugar for the method call, so they are still statically checked at compile time. So using them your code is every bit as safe if you simply called the corresponding methods yourself.

Below is a table of operators and the corresponding trait that allows you to implement support for them.

| Operator | Example | Explanation | Trait |
|----------|---------|-------------|---------------|
| `!` | `!expr` | Bitwise or logical complement | `Not` |
| `!=` | `var != expr` | Nonequality comparison | `PartialEq` |
| `%` | `expr % expr` | Arithmetic remainder | `Rem` |
| `%=` | `var %= expr` | Arithmetic remainder and assignment | `RemAssign` |
| `&` | `expr & expr` | Bitwise AND | `BitAnd` |
| `&=` | `var &= expr` | Bitwise AND and assignment | `BitAndAssign`
| `*` | `expr * expr` | Arithmetic multiplication | `Mul` |
| `*=` | `var *= expr` | Arithmetic multiplication and assignment | `MulAssign` |
| `+` | `expr + expr` | Arithmetic addition | `Add` |
| `+=` | `var += expr` | Arithmetic addition and assignment | `AddAssign` |
| `-` | `- expr` | Arithmetic negation | `Neg` |
| `-` | `expr - expr` | Arithmetic subtraction | `Sub` |
| `-=` | `var -= expr` | Arithmetic subtraction and assignment | `SubAssign` |
| `/` | `expr / expr` | Arithmetic division | `Div` |
| `/=` | `var /= expr` | Arithmetic division and assignment | `DivAssign` |
| `<<` | `expr << expr` | Left-shift | `Shl` |
| `<<=` | `var <<= expr` | Left-shift and assignment | `ShlAssign` |
| `<` | `expr < expr` | Less than comparison | `PartialOrd` |
| `<=` | `expr <= expr` | Less than or equal to comparison | `PartialOrd` |
| `==` | `expr == expr` | Equality comparison | `PartialEq` |
| `>` | `expr > expr` | Greater than comparison | `PartialOrd` |
| `>=` | `expr >= expr` | Greater than or equal to comparison | `PartialOrd` |
| `>>` | `expr >> expr` | Right-shift | `Shr` |
| `>>=` | `var >>= expr` | Right-shift and assignment | `ShrAssign` |
| `^` | `expr ^ expr` | Bitwise exclusive OR | `BitXor` |
| `^=` | `var ^= expr` | Bitwise exclusive OR and assignment | `BitXorAssign` |
| <code>\|</code> | <code>expr \| expr</code> | Bitwise OR | `BitOr` |
| <code>\|=</code> | <code>var \|= expr</code> | Bitwise OR and assignment | `BitOrAssign` |

## deref

One common operator is known as the “deref” operator. It's trait is defined as:
```rust ,skt-default
pub trait Deref {
    /// The resulting type after dereferencing.]
    type Target;
    /// Dereferences the value.
    fn deref(&self) -> &Self::Target;
}
```
The idea of deref is to allow access to a wrapped inner object. You may have heard invoking a method on a member variable is: “this.foo.bar()” refered to as “dereferencing” the member variable. This is where the name comes from. ‘Box’ implements 'deref’. This allows you to write
```rust ,skt-main
# struct Foo();
let b : Box<Foo> = //...
# Box::new(Foo());
let f : Foo = *b;
```
or even more simply:
```rust
# #![allow(dead_code)]
# #![allow(unused)]
struct Foo {
    a : u32,
}
fn function(foo: &Foo) {
    //...
}
pub fn main () {
    let b = Box::new(Foo {a: 4});
    function(&b);
}
```
This allows you to treat a 'Box<T>’ as though it was a T most of the time, which makes working with Box a lot more convenient.

Similarly `MutexGuard` implements deref. So you can just write an method like:
```rust ,skt-default
use std::sync::Mutex;

struct MyObject {
    data : Mutex<Vec<i32>>,
}
impl MyObject {
    fn add_data(&self, new_item: i32) {
        let mut data = self.data.lock().unwrap();
        data.push(new_item);
    }
}
```
Here `data` is a `MutexGuard`, and it will automatically release the lock when it goes out of scope, but `data` can use the methods on the guarded object directly without having to unwrap it.

Deref can be used with assignment. So with MutexGard you could write:
```rust ,skt-default
use std::sync::Mutex;

struct MyObject {
    data : Mutex<Vec<i32>>,
}
impl MyObject {
    fn add_data(&self, replacement: Vec<i32>) {
        let mut data = self.data.lock().unwrap();
        *data = replacement;
    }
}
```
Suppose you want to add some functionality to a type, the easiest way is to wrap it and add the functionality to your wrapper. For example to make an existing type sortable, in Java it would look like.
__
In Rust it would look like __. Of course you probably want to get the original object out of your wrapper so you can actually use it. In Java you might use a getter. In Rust you would use Deref. Deref is the trait the corresponds with the ‘*’ prefix operator. So you can do this __.

The deref method will automatically be invoked by the compiler any time an object is passed to a method expecting a type that it can deref into. This means for example you could pass an object like this __ because the method is implemented for _ and the wrapper will automatically have its deref method called and the result passed to the function.

A good example of this is String. Strings much like arrays support slicing. So you can take a view of a subset of a string. This is similar to the way in Java you can have a ByteBuffer which provides a view of a subset of a byte array. String implements Deref for str. Given a borrowed string it returns a borrowed slice containing the whole string. So you invoke deref explicitly or just pass a String anywhere a string slice is expected, the conversion is done for you automatically. This is even true when invoking methods on the object. Say you have a borrowed String. You can call ‘.chars()’ and get the characters in the string. Even though the ‘chars’ method was actually defined for the string slice.

Now imagine that for your own wrapper types, it means they be generic to what they contain an still allow the caller to invoke whatever methods they need on the wrapped object with no syntactic overhead safely without any ambiguity. (Cool right?) It should be noted that because Deref is automatic, it is not meant for costly conversions. For that you should use the ‘From’ or ‘Into’ traits. Where the conversion is more explicit.

## Drop
There is one more common operation that can be overloaded, but it does not have any explicit syntax. That is `Drop`.
Because Rust's memory management is deterministic it is possible to have a method run right before a object is dropped.

This can be done by implementing the `Drop` trait, which is defined as:
```rust ,skt-default
pub trait Drop {
    fn drop(&mut self);
}
```
This is implemented on Sockets and Files. Drop is in some ways similar to Java's `AutoClosable` which can is used to make
"try-with" resources that have their close method called at the end of a try block.

Drop is different in that the that the cleanup method `drop` is always called when the object goes out of scope.
So there is no need for a special block or syntax required. It also makes it impossible to forget to use the method/syntax.
This is why they say [Rust Means Never Having to Close a Socket](https://blog.skylight.io/rust-means-never-having-to-close-a-socket/).

Implementations of Drop cannot fail and shouldn’t block. So sometimes is there are still `close()` methods in case for
example an RPC needs to be made.

A good example of where this is used is `Mutex`, these are used for synchronization and will be discussed more in the next chapter. The mutex returns a mutexguard object. This is a wrapper object that holds the lock while it is in scope. The mutexguard locks the mutex when it is created and unlocks it when it goes out of scope. IE:__ Because he only to access the value is though the mutexguard it is impossible to misuse the interface. If you have access to the variable you must be holding a lock. There is no way to forget to either acquire or release the lock.

If in Java an exception is thrown from inside of a synchronized block, (assuming it is not caught inside) the lock will be released and the exception will propagate up the stack. Certainly this is desirable in the case where the exception is being deliberately thrown. But in the event of an Error this might be undesirable. The lock will be released and other threads will proceed to use it even though it only completed half of the critical section. This might be harmless or it might leave the object in a bad state and trigger weird problems in other threads. This is an example of why treating unexpected and expected errors in the same way is not a great idea.

Rust resolves this by distinguishing between an panic and a normal error. It is expected that critical sections may return errors. If they do, the lock is released normally and other threads can use the guarded resource. Critical section however are not expected to panic. If one does, in the destructor of the mutexguard it will detect the panic and “poison” the lock. This will mean any thread attempting to acquire the lock will receive an error and won’t be able to access the guarded resource. This acts as a safety check against the data being left in an inconsistent state.
