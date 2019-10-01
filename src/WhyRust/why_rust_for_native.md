# Goals
We would like to develop a low-level library that can be used to construct multiple higher level libraries in several different languages.

To do this we need the library to be very robust, well defined, and able to interoperable well with other languages.

## Requirements for cross language calls

Cross language calls, add many additional requirements that aren't generally present when you are working within a single language.

Consider how two different languages might differ
* What character encoding is used for strings?
    - Does conversion need to be done? Can the foreign string type be instantiated?
* How does error handling work in the calling language? 
    - Mapping one languages exceptions to another can be complicated
* Does data need to be aligned?
    - Calling conventions and object alignment need to agree
* Are there OS/Target specific requirements?
    - A Python library might expect its dependencies to be architecture agnostic which can be complicated for a native library.
* What allocator is used to allocate memory?
    - Languages like Go and Java expect memory to allocated in a particular way.
* Are there restrictions on threading?
    - For example Python's interpreter lock
* Is the memory garbage collected, in one or both languages? 
    - Is there an issue handing data from one heap to another?
* Can the implementation safely hold onto objects passed after the call has returned?

As a result libraries that are intended to be used by another language tend to benefit from the maximum level of control
and precision in the specification of their interfaces.

# Functions signatures

## Java
Consider you want to write a function similar to the following Java:
```java
interface Writer {
    public void write(ByteBuffer src) throws IOException;
}
```
This signature convays a lot of information. It provides the parameters, the return type, and an exception for a way it can fail.
Because Java is memory managed, there is no need to worry about whose responsibility it is to free `src`. 

It also has some ambiguities such as will the method modify `src`. You might guess it would updated the position but you'd have 
to read the documentation to know for sure. It also could throw additional RuntimeExceptions.

## C++
Compare this to a similar C++ function:
```
    size_t write(const void * ptr, size_t size, size_t count);
``` 
This function similarly has inputs and a return value.
It however has several additional ambiguities:
* C++ is not memory managed so documentations is needed to determine who should free the memory pointed to by `ptr`.
* While C++ has exceptions they are not used here (this is common), instead the return value must be checked which can be forgotten
* `size` and `count` describe how much data can be read from `ptr` but nothing actually enforces this. If the passed sizes are too large it can be a security issue.
* Like all methods in C++ it is not clear or enforced by its signature if it is Thread-safe, Signal-safe, or Exception-safe.

A signature from `C` or `Go` would look nearly identical and have these same ambiguities.

## Rust
[Rust has become an option for writing native libraries.](https://youtu.be/_wy4tuFEpz0) because of its focus on control an safety.

Many people in the Python, Ruby, and JS community have 
[adopted Rust as the preferred way to write "performance critical" functions in native code](https://youtu.be/FYGS2q1bljE)
that can be invoked as part of high level libraries. 

Compare the a above to a signature in Rust:
```rust
pub trait Writer: Sync {
  fn write(&self, buf: &[u8]) -> Result<(), WriteErr>;
}
enum WriteErr {
    NoSuchStream(),
    CouldNotContactServer(),
    //...
}
```
This signature encodes a lot more information. Here the caller knows:
* It is safe to call `Writer` from multiple threads concurrently.
* The `buf` argument will not be modified by `write`.
* The `write` method won't retain any refrence or pointer to any data in `buf` after the call returns.
* It is the caller's responsibility to free `buf`, the implementation will not do it.
* The `Result` provides an exhaustive list of the ways the method can fail, which can all be handled explicetly.

Similarly it allows for control of how memory is allocated and who is responsible for deallocating it.

###Popularity

Safety combined with Rust's great packaging tools has made it the language of choice for writing small reusable libraries.
As a point of compairson: Go has more total developers who know the language, but Rust has more [open source libraries available for download](http://modulecounts.com) 
and is adding new ones more quickly than Go. This may be important depending on how frequently you need to look for a library to depend on.

###Tradeoffs
Explicitness is a double edged sword. It makes writing code a lot more explicit which may slow down writing code when there 
only ever one implementation and one call site and one author. To make writing code fast, languages like 
`Python` and `Go` have chosen to omit unnecessary specification detail. 

When there are many callers and a function that gets reused and maintained for a long time, the maintenance burden makes it
worthwhile to spend the extra time up front, and use the compiler detect problems. In short: 
any savings in programming time, must be weighed against the bugs that could have been prevented.

For the case where we intend to implement a single high performance library and call into it from many languages, 
specifying additional information is well worth our time. 

A longer description of Rusts values is [here.](https://youtu.be/2wZ1pCpJUIM)

###Sefety
Rust uses the additional information to provide additional guarantees. Specifically, if code compiles it is guaranteed to:
* Comply with all aspects of the interface (or it will be a compiler error)
* Be free of data races
* Have no null pointer exceptions
* Have no class cast exceptions
* Have no iterator invalidation bugs
* Have no use after free bugs
* Have no double free bugs
* Have no dangling pointer bugs
* Not have any memory leaks
* Is not susceptible to buffer overflow or similar attacks

This is a direct result of how [Rust deals with shared mutable state](https://youtu.be/IwjlCxwcuIc)
which is its most unique feature. [And is levaraged in many ways to provide strong guarentees](https://youtu.be/cDFSrVhnZKo)






