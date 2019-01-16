# Error handling
Java has both checked and unchecked exceptions. This is been criticized a lot of people both inside and outside Java community how are the original ideas actually sound. Checked exceptions make sense where the exception is expected to be handled by the immediate caller. unchecked exceptions make more sense were the exception just isn't anticipated at all, for example because it results from a situation that would require a programming error. this leads to a pattern that's rather elegant in theory: known failure modes must be handled explicitly, while unknown ones propagate up the stack as far as necessary until there's some general catch-all block that deals with them. 

Java's implementation of this however isn't ideal. Part of the problem is in early versions of Java an implementation had to declare that a method throws an exception if it implemented an interface that declared such an exception. While this is no longer the case, it is the reason why ByteArrayOutputStream still throws IOException. There are also a couple of cases where Java defined things is a very awkward way, like that InputStream's and OutputStream's close method throws, or that InterruptedException is both checked and catching it resets interrupted status. Finally and more subjectivity, RuntimeExceptions and checked exceptions FEEL too similar. It's too easy to think of them as part of the same thing, and not realize the massive difference between ordinary errors and cases that truly cannot be handled, because they arise from bugs in the program itself.

All of this combined with a lack of a concise way to rethrow one exception as another lead to a large backlash against checked exceptions. This is somewhat unfortunate, because similar to how the backlash against some annoying corner cases with types resulted in the popularity of dynamic typed languages, the backlash against the inconveniences of checked exceptions directly lead to C# removing them and using a strictly runtime exception model. Which while more convenient than Java's model was a step back in terms of ensuring correctness.

Fortunately Rust has found a way to resolve these issues. Rust's solution is quite different from Java's. But it is similar in that there are two types of error handling. The “normal” way, which is used for expected errors provides strong guarantees that the error will be handled. Rust's error handling syntax is very lightweight. It doesn't require writing try/catch blocks all over the place, and provides a seamless way of converting between different exception types and converting to the “unrecoverable” type (analogous to Java's runtime exceptions) allowing a way to “bail out” instead of handling the error, if there isn't a way to do so. Below is an example __. (See Joe Duffy's “The error model” and use table)

In Java it is common to use Preconditions. The pattern in Rust is to use assertions. So the following are idiomatically equivalent 
```java
class Decoder {
  //...
  public Message decode(String message) {
    Preconditions.checkNotNull(message);
    Preconditions.checkArgument(!message.isEmpty(), "Cannot decode an empty message.");
    //...
  }
}
```
and
```rust
# struct Message();
struct Decoder {
  //...
}
impl Decoder {
  //...
  pub fn decode(&self, message : &str) -> Message {
    //No need for a null check because there are no NPEs in Rust.
    assert!(!message.is_empty());
    //...
  # Message()
  }
}
```
If instead of panicking you just want to return an Error, use the 'ensure!’ macro from the Failure crate. 
{{#playpen failure_example.rs}}

One problem with Java's exceptions is they don't work with things like Futures or Java Streams very well. This is because these interfaces need to be generic so the type information in the exception is lost, and they are likely running from a callback or threadpool so the stacktrace is not helpful. This eliminates two of the main benefits of using exceptions. Fortunately Rust's model overcomes both of these problems. So where in Java you would have to write something like this __ and if it failed you would get a stack traces that looked like __, whereas in Rust you could just write __ and if it failed you would get a stacktrace that looked like __.

To add a new exception type on a public interface, in Java you would either have to make the new exception Runtime. (Which may or may not be desirable depending on the circumstances) Or you can subtype an existing exception. In Rust the pattern is to use an error Enum. For example:
{{#playpen failure_example_2.rs}}
This way all the errors are explicitly enumerated and they can either be handled all the same or individually by the caller. If you use a non-exhaustive Enum like
```rust,ignore
#[derive(Debug, Display, Fail)]
#[non_exhaustive]
enum Unprocessable {
  MalformedInput,
  BackendUnavailable,
  UnknownException,
}
```
Then new types of failures can be added in the future without breaking compatibility. (Because in a matching on the Enum will require a default branch) Ie:
```rust
# enum Unprocessable {
#   MalformedInput,
#   BackendUnavailable,
#   UnknownException,
# }
fn failing_function() -> Result<(), Unprocessable> {
  //...
  return Err(Unprocessable::MalformedInput);
  //...
}
match failing_function() {
  MalformedInput => println!("Bad input"),
  _ => println!("Unexpected error"),
}
```

In Java there are checked exceptions such as IOException and unchecked exceptions such as Error. Rust has analogous concepts. Panic is a similar to an Error in Java. It is assumed to be unrecoverable. It is sort of a controlled crash. In the event of a panic rust will gracefully unwind the stack similar to Java. It runs destructors (which works like a finalize block in Java). However it will not release locks, as this could mean that only a part of a critical section was executed and the data is left in an inconsistent state. (Other threads trying to acquire the mutex will get error when they call lock()). 



In addition Rust has error types these work like a return value but with some special syntax surrounding them. This ends up working a bit like checked exceptions in Java. 

Exceptions like many things in Rust are a crate that is imported, rather than built into the syntax itself. The preferred implementation is one called “Failure”. much like Java it allows you to create your own exception types chain exceptions together when one causes another, and provides stack traces. However exceptions in Rust work differently than they do in Java. In fact the exceptions isn't really the right word they work somewhere between a return value at an exception. They do however have a very compact syntax which is helpful. 

In Java you might write:
```java
private String readFileIntoString(String filePath) throw IOException;
```
The equivalent Rust would be:
```rust
# use std::io;
# pub trait Example { 
fn read_file_into_string(path: String) -> Result<String, io::Error>;
# }
```
Notice that the exception is actually part of the return value. Specifically the function either returns a `String` or an Exception. If that was all there was, it still would be a step up from languages like c or go in that you cannot forget to check the value. But having to manually unwrap each return value and usually re-throw the exception at every stack frame would be very tedious and seem down right primitive compared to Java. Fortunately this is not the case. You can use a special operator the question mark. If you have code like the following: 
```rust    

```
The `?` Was simply rethrow the exception. But that's not all, you can also convert between exception types automatically changing the new exception to the old one. For example you might write the following code in Java: __. This is very verbose and not very clear in terms of its flow. However it is equivalent to the following Rest code: __. as you can see this is both very explicit and compact it shows the flow control very nicely you can see the points where function can exit, and it does so without any extraneous indentation Constructors or unnecessary blocks. Just to show a more sophisticated example the following are equivalent: __ and __. The exception generated will look like __ and __. but notice the rest code didn't have nice line numbers in backtrace is for the intermediate functions. fortunately to enable this is quite simple you just set the environmental variable RUST_BAKTRACE=1. Then it will print the following __. Because this doesn't add any overhead in the non-failure case (and is fairly cheap even in the error case), and increases debuggability I simply leave this on all the time.

In Java a common approach is to have use multiple subclasses of a common exception. In Rust the pattern is to use an enum. So this Java function _(io)_, would be written like this in Rust. __. So in Java a caller handling these together would just catch _ or would handle them separately with multiple catch blocks __. In Rust they can be handled collectively __ or via a Match statement __. However this code is messy and verbose. So Rust allows you to instead write __. This is exactly the same as the above code. The ? Operator unwraps the result object if the call was successful, otherwise it returns an error, and if the method’s error type is different it will automatically call __ to wrap or convert the error. This pattern is commonly used in conjunction with a crate called Failure which automatically generates a lot of the boilerplate code for you and provides backtraces that can we accessed just like in Java 10. __. The advantage of Failure backtraces as opposed to Java's is they automatically work across threads. __ 

Another common pattern in Rust that gets exceptions even further out of the way is to Alias error. For example __. Then all the methods can just have __ in their signature and not have to type the exception signature over and over. 

## Destructors
  • Destructors
    ◦ Drop trait
    ◦ Mutex
    ◦ Refcell
    ◦ Can't fail to close socket or file

Programming in Rust means never having to close a Socket…. ...


Another good example of where destructors are used is Mutexes, these are used for synchronization and will be discussed more in the next chapter. The mutex returns a mutexguard object. This is a wrapper object that holds the lock while it is in scope. The mutexguard locks the mutex when it is created and unlocks it when it goes out of scope. This works similarly to a try-with resource in Java. IE:__ Because he only to access the value is though the mutexguard it is impossible to misuse the interface. If you have access to the variable you must be holding a lock. There is no way to forget to either acquire or release the lock.

If in Java an exception is thrown from inside of a synchronized block, (assuming it is not caught inside) the lock will be released and the exception will propagate up the stack. Certainly this is desirable in the case where the exception is being deliberately thrown. But in the event of an Error this might be undesirable. The lock will be released and other threads will proceed to use it even though it only completed half of the critical section. This might be harmless or it might leave the object in a bad state and trigger weird problems in other threads. This is an example of why treating unexpected and expected errors in the same way is not a great idea. 

Rust resolves this by distinguishing between an panic and a normal error. It is expected that critical sections may return errors. If they do, the lock is released normally and other threads can use the guarded resource. Critical section however are not expected to panic. If one does, in the destructor of the mutexguard it will detect the panic and “poison” the lock. This will mean any thread attempting to acquire the lock will receive an error and won’t be able to access the guarded resource. This acts as a safety check against the data being left in an inconsistent state.

