# Lifetimes

  * http://cglab.ca/~abeinges/blah/too-many-lists/book/second-iter.html 
  * Normally figured out by the compiler for you
    * Here is what it is doing
    * Only applies to borrows, not to owned items, and not the item being borrowed.
      * This means that for instance you can specify a single ‘a on one of the parameters and the return value to indicate that the returned value came from that parameter and therefore the borrow should last as long as the parameter’s it came from.
      * Similarly you can have a case where the value could come from more than one input, you can just add a parameter to all the relevant inputs __. This will result in compiler checking that the original objects this function is borrowing live at least as long as the result returned from this function. 
      * Notice that we don’t actually have to know where the inputs came from or have any idea how long they will live. The compiler will just check and make sure things are OK.
    * Specified with a lowercase letter to distinguish from T. 
    * Convention is to start with ‘a and then ‘b etc.
    * Unlike a generic value T they don’t have any meaning. They are only needed to distinguish one value from another. 
      * Example 
  * Statics lifetimes - live for as long as the program.
    * &str is often this. Because string constants are part of the program itself and hence by definition exist for the lifetime of the program. You can of course borrow them, and when you do they have a static lifetime.
  * Destructors
  * try/with replaced by destructor
    * Examples
  * Destructors cannot fail and shouldn’t block. So sometimes close exists.
