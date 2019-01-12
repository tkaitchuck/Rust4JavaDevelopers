# How Rust makes you a better Java programmer

  • “Data structures make it easy to add new functions without changing the structure. Where as objects allow adding new classes without changing the functions.” Oo makes it hard to add new functions, but traits allow this.
  • Shared mutable state is the source of most bugs.
    ◦ Oo languages limit sharing by hiding an impl behind the interface of an object. This is vastly better than the alternative. But doesn't fully hide things because … 
      ▪ Indirection allows hidden changes
    ◦ Functional languages remove mutable state. That prevents many problems, but it makes it much harder to do certain things. Which introduces its own bugs. Or you end up writing a lot of code to make immutable state look like mutable state. Which ends reintroducing some of the problems that were prevented.
    ◦ Rust allows mutability without sharing, or sharing without mutation, but never both at the same time.
  • The expression problem: The goal is to define a datatype by cases, where one can add new cases to the datatype and new functions over the datatype, without recompiling existing code, and while retaining static type safety (e.g., no casts)
  • How rust makes you better at Java
    ◦ Ownership
      ▪ Nested sync
      ▪ Modifying an object after passing.
      ▪ Graphs
    ◦ Composition over inheritance
    ◦ Circular references
      ▪ Are a problem, for c..
      ▪ Are not needed most of the time. 
      ▪ Usually for a callback. That is better handled via 
        • FUtures 
        • …
      ▪ Don’t fear copies. (10x java copy speed)
        • to_owned() from a ref 
    ◦ Solid principles

Rust’s strict enforcement of rules makes a lot of bad patterns particularly hard to write. This can be a good thing if you recognise when something is going wrong, because it stops you from making a design mistake. <Insert don’t fight the borrow checker>
For an example of this see the article “Too many linked lists” or the Rustconf 2018 closing keynote “__”.

## Thread safety provided by ownership
  • Race conditions and ownership.
    ◦ Race by releasing lock
    ◦ Compare java and rust
  • Deadlock
    ◦ In one class (obvious)
    ◦ Cyclic dependency
      ▪ Final initialization prevents
        • Not always practiced in Java
        • Often people do realize memory model implications
        • Rust pattern is really akward and discouraged.
        • Rust compiler correctly identifies memory model problem and forces you to grapple with it.
    ◦ Object passed in during call
      ▪ Java rules should prevent
        • Often not practiced.
        • Not as easy to spot because you can't tell at the call site. If code not always following rules and classes are mixed.
      ▪ In Rust still possible. Example: a owns b, c. B holds c. A passes B to C by reference. B and C both have inner objects guarded by a mutex. The scope of the guard extends over calls to each other.
        • Could be prevented by the old fashioned advice to keep the scope of locks small.
        • Should be aware of such cycles. Not always obvious. Rule of thumb is not to invoke methods on some other argument while holding a lock. For example __
        • In general in Rust if you are using a mutex over two objects you are doing it wrong. It is reasonable to make updates atomic, but the lock is in the wrong place. It should just wrap the thing that needs to be made atomic. So instead just hold onto the inner objects directly and make the method take a &mut self. Then wrap the object in mutex one level up.
    ◦ Call back
      ▪ Never invoke a callback or complete a future while holding a lock. (In any language)
        • Really blatant violation of invoking a method on a passed object while holding a lock.
      ▪ This is the ultimate narrow the scope of a lock. Who knows what a callback includes!
      ▪ In Rust this is not a problem for futures as, unlike Java's they don't work as a callback. They get run by the executor. It's like if in Java you invoked __ every time instead of __. This is a safe way to avoid holding a lock in Java. Unfortunately it does have some overhead, so most codebases can't afford to simply have the rule that it should be used every time. Fortunately Rust have found a way to use their ownership model to make their futures Radically more efficient. For example __benchmark__.


How to write thread safe code in Java….

These concurrency rules help deal with cases where you have multiple independent tasks running in parallel, or where you have a shared state the needs to be coordinated between two tasks. 
Another pattern is message passing. This is also often used in Java using Queues such as the ArrayBlockingQueue or the LinkedBlockingQueue or the synchronunsQuntne. These are totally compatible with the model, they should just be considered an…..


When I was first learning Rust, after reading the Rust book, I started reading the trust-dns codebase to make sure I understood things. I remember being shocked when I saw this: https://github.com/bluejekyll/trust-dns/blob/master/https/src/https_client_stream.rs#L38 I saw and Arc around the String and was immediately afraid. Coming from many years of managing thread safety in Java, the idea that a mutable class (String) was being shared across threads (why else would it be wrapped in an Arc), and this object was wrapped in a struct that was going to be returned from a function (Implied by the #must_use annotation) nearly sent me into cardiac arrest! (And if it didn’t deeply frighten you, let me assure you, your code is riddled with thread safety errors).

In just 7 lines of a struct definition, it is violating every possible tenet of writing thread safe code at the same time. And yet… it’s ok. String is itself mutable, Arc won’t allow any mutable borrows of the value unless there is only one reference to it. And the string is owned by the Arc, so there can’t be code anywhere that could have a direct reference to the string. So even though String can be modified, and is shared, and the struct is shared across threads, and is returned out of a public method, any code using that string can treat it as immutable. It will never change out from underneath it.

After spent years working strictly following the discipline above, teaching it to all of my co-workers and carefully reviewing code for compliance, I always thought having the compiler enforce thread safety was just a matter of automating this work. It would be just like writing code in Java, but the conventiones would be formalized, and the review would be done by the compiler. What I didn’t anticipate was that having safety enforced by the compiler would open up new possibilities. A pattern like the one in the trust-dns client would never be allowed, because no sane person would ever want to use it in Java, because it would be hopeless to ever get right. But in Rust, it was just nonchalantly written by someone who didn’t even think about it because they *knew* the compiler would keep things safe. I think that’s pretty cool.
