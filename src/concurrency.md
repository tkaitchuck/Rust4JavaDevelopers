# Concurrency

  • Futures_cpupool
    ◦ Futures chaining
    ◦ Async / await.
    ◦ Futex
  • Async example w/ futures
    ◦ Select and join compared to and and or in Java 
    ◦ Futures with streams (.buffered)
    ◦ Performance
  • Async! Macro
  • Arc
  • Queue example / channels
    ◦ Send attribute
    ◦ Select
    ◦ using channels doesn't prevent race conditions because Go lacks immutability. In Java the same problem exists. In Rust it ownership is explicit so it’s safe
  • Turn a sync API into an async one

In Java you would synchronize the code like this.__ In Rust you synchronize it like this.__  In both cases the lock is guaranteed to be released at the end of the scope. In Java’s case it is because a special scope is being introduced. In the case of Rust, there is no special language feature it just uses the normal destructor to release the lock. This means a new scope does not need to be introduced. Of course you can add one line this __ if you want to reduce the scope a lock is held. (This could also be done with a private method). Another common pattern is to use a loop conditional expression __ as this has a very small scope.

Locks in both languages work the same way, if one thread holds the lock, another thread attempting to acquire the same lock will block. 

The mutex object returns a MutexGard object which works like a RefCell. It allows for mutablity borrowing the data but does not allow for taking ownership.

Safety Monitor: Notice that the type perfectly reflects the guarantees. The mutex is guaranteeing only a single thread can hold the lock at any given time, and hence follows the contract of mut. And the value is borrowed so it can't be accidentally stored into some other object which would allow it to be accessed outside the scope of the lock. 

In Java the pattern is that locks should live inside of the class and it is the responsibility of the implementation to implement the locking. This makes sense as the code is relatively compact and all located in a single file which makes it easier to validate. In Rust the pattern is to do synchronization outside. This has a few advantages: it allows callers to make multiple calls under the same lock, so two or more calls in order can be done atomically together without using a second lock. It also means the class can be written once without thinking about thread safety and added by the caller if needed. Applying this pattern to Java would be a bad idea because it would be very hard to verify the lock is held everywhere the data is being accessed. By putting all of the places you need to check into a single file, you can verify it is correct. This is not a problem with Rust because the correctness will be enforced by the compiler. Applying the Java pattern to Rust is also a bad idea. Because other someone else in another module can add methods to your objects neither the compiler nor any of the standard tools will allow a pattern where all of the methods on a particular object are expected to do something.


In Java it is important to encapsulate state for ownership and in turn thread safety. In many functional programming languages the pattern is to do the reverse and have the state passed into the function. The advantage of this is it makes the program very easy to test. Example:
___
In Rust because the ownership contract is explicit it becomes easier to write more testable code around state transitions. Example:
__
This retains the strong ownership and safety guarantees of the Java version while improving the testability.
This can be made explicit using types. If we write it like this the compiler will enforce the correct pattern on callers.
__
Notice that as far as the caller is concerned they are dealing with a single class, but internally we have made it such that only the valid sequence transitions can occur.


Here is an example of how to make a thread safe version of a Map in both Java and Rust. __ In Java we use Collections.synchronizedMap(). The implementation of this looks like __. There are a few things to notice. First, this is very specific to Map. It can't even deal with a nested data structure. So the standard library needs a different one of these for every collection and have to write your own class like this for every data structure you create. Not great in terms of code duplication. Additionally it does not solve the whole problem. The method iterator() and stream() note this in their Javadocs. The caller is expected to grab the lock like this __. This is not ideal because it breaks the abstraction and it's not safe because there is nothing to check and make sure the caller actually does this. Finally the caller itself could modify the map when iterating over it. This would result in incorrect behavior even in a single thread. None of these sort of mistakes are possible in the Rust version as they would all be compile errors. (The iterator method borrows the collection, so it cannot be modified elsewhere before the iterator is dropped.) __

There is a popular crate called Rayon that takes this one level further. If you import Rayon it adds a concept of a parrallel iterator, to automatically provide data-parallel operations on most data structures. For example if you have __ you can just change ‘.iter’ to ‘.par_iter’ like so __. Then the code will run in parallel on multiple threads. All the needed synchronization and coordination is managed by Rayon, and all safety guraentees remain in place. So if you accidentally tried to modify an outside variable without synchronization like this __ the compiler will catch it and produce and error _.

Another elegant albeit rarely used pattern in Rust is Cow. COW stands for copy on write. Cow is similar to Java’s CopyOnWriteArrayList or CopyOnWriteArraySet but it isn't specific to a single data structure. It is generic and can wrap any cloneable object. It will clone the object if it gets modified while there is another reference outstanding. Which means if cow is wrapping a list that is modified in a loop, the list will be copied at most once.


There are two important traits 'send’ which indicates a type can be sent from one thread to another. And ‘sync’ which indicates a type can be accessed by multiple threads at the same time. These are Auto traits. So the compiler will automatically work out for each type if it is one or both of those. So while you normally don't need to think about them, they can be useful if you want to declare this requirement on an interface. For example __. Here the interface explicitly requires that the Type passed in be sendable between threads.


In Java if you had an application where you were dealing with users and users are allowed to perform certain operations once they have been authenticated. Then you might define two different classes _ and _. Then your method signature could like _ and AuthenticatedUser could look like this _. The advantage of this approach is there is not an “isAuthenticated” boolean that you could forget to check, or that could be accidentally set through some other code path. Here the only way to invoke the sensitive method is to have an AuthenticatedUser and the only way to get one is to call Authenticate. This is in effect using Java’s type system to enforce an invariant in your code. These sort of techniques allow Java code to offer stronger security guarantees than more dynamic languages like Ruby or Python. It Rust this pattern is even more common because in addition to types, Rust's stronger notion or generics, Traits, Lifetimes, and ownership allow many different types of guarantees to be enforced. This is worth thinking about when you are designing interfaces. A great example of this is the “mpsc” class in Rust's standard library.

In CS theory there is a really clever and very low overhead lock-free algorithm for transferring data between threads. However it only works if there is only a single thread that is receiving the data. This is still handy because it is common to have many worker threads reporting their results back to some master thread. This algorithm was actually well known at the time that Java was designing its concurrent collections. But it wasn't used. It wasn't because it would be very awkward to explain. Imagine reading the docs to a class like LinkedBlockingQueue and seeing a long explanation of which threads were allowed to call which methods. Because this would simply be too error-prone Java only provided a less efficient but more general algorithm. The crux of the problem was that Java just couldn't represent the restrictions on the object within the language itself. This isn't true in Rust. You can see an implementation of this in std::sync::mpsc (mpsc is an acronym for multiple producer single consumer). FOOTNOTE:(This isn't the only or best way to send data between threads. If you are looking for a more flexible alternative check out crossbeam-channel)

(While the implementation of mpsc isn't a big deal, it is a useful example to show how complicated restrictions can be modeled). To create sure a channel you call _ and it returns two objects a sender and a receiver, where items sent to the sender go to the receiver. The sender implements clone which means you are free to make multiple copies of it. (The copies are similarly connected to the receiver) Both sender and receiver implement the trait Send but not the trait Sync. This means they can be passed to another thread but not used by multiple threads at the same time. (This is enforced by the compiler) The channel is closed automatically when all the producers or the consumer are dropped. While this pattern looks a little odd, it creates an interface that is impossible to use incorrectly. So even though the requirements for this algorithm are oddly specific, Rust allows it to be implemented in a way where any bug in the way it is used will be a compilation error. This is the Rustacean ideal. 