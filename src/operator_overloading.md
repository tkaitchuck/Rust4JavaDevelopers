# Operator Overloading

  • Size() length, get(), []

Traits are also used to what in other languages is called “operator overloading”. For each operator like ‘+’ for example there is an associated trait, which has a method that can be implemented. If an object has an implementation of the trait, you can use the operator as a shorthand for invoking that method. For example in Java it is very common to implement Compairable. However only primitive types can use the ‘<’ and ‘>’ operators. So you have to write:
__
Where as in rust if your type implements the Ord trait you can just write:
__
Note that Ord compares the object against others of the same type so unlike Java you don't have to worry about a class cast exception at runtime.

Unlike C++ or Scala all of the operators that can be overloaded and the signature of their corresponding methods is pre-defined by the language. Because implementing a trait is only allowed in the package that defined either the trait or the type you can add support for operators to your own types, but you can't add or alter their behavior on someone else's types. So you can't add you own “pope operator” that looks like “<+|:-)”. And operators can't be used to do anything crazy or surprising.

Operators are just syntactic sugar for the method call, so they are still statically checked at compile time. So using them your code is every bit as safe if you simply called the corresponding methods yourself. 

Below is a table of operators and the corresponding trait that allows you to implement support for them.
__


One common operator is known as the “deref” operator. It's trait is defined as __. The idea of deref is to allow access to a wrapped inner object. You may have heard invoking a method on a member variable is: “this.foo.bar()” refered to as “dereferencing” the member variable. This is where the name comes from. ‘Box’ implements 'deref’. This allows you to write __ or even more simply __. This allows you to treat a 'Box<T>’ as though it was a T most of the time, which makes working with Box a lot more convenient.

Similarly MutexGard implements deref. So you can just write __. And use the methods on the guarded object directly without having to unwrap it. 



This even holds for assignment. So with a Box you could write __. Or with a MutexGard you could write __.

Suppose you want to add some functionality to a type, the easiest way is to wrap it and add the functionality to your wrapper. For example to make an existing type sortable, in Java it would look like __. In Rust it would look like __. Of course you probably want to get the original object out of your wrapper so you can actually use it. In Java you might use a getter. In Rust you would use Deref. Deref is the trait the corresponds with the ‘*’ prefix operator. So you can do this __. 

The deref method will automatically be invoked by the compiler any time an object is passed to a method expecting a type that it can deref into. This means for example you could pass an object like this __ because the method is implemented for _ and the wrapper will automatically have its deref method called and the result passed to the function. 

A good example of this is String. Strings much like arrays support slicing. So you can take a view of a subset of a string. This is similar to the way in Java you can have a ByteBuffer which provides a view of a subset of a byte array. String implements Deref for str. Given a borrowed string it returns a borrowed slice containing the whole string. So you invoke deref explicitly or just pass a String anywhere a string slice is expected, the conversion is done for you automatically. This is even true when invoking methods on the object. Say you have a borrowed String. You can call ‘.chars()’ and get the characters in the string. Even though the ‘chars’ method was actually defined for the string slice. 

Now imagine that for your own wrapper types, it means they be generic to what they contain an still allow the caller to invoke whatever methods they need on the wrapped object with no syntactic overhead safely without any ambiguity. (Cool right?) It should be noted that because Deref is automatic, it is not meant for costly conversions. For that you should use the ‘From’ or ‘Into’ traits. Where the conversion is more explicit. 
