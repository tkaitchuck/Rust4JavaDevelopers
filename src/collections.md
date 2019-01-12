# Collections

  • Vec
  • HashMap
  • Statically constructed collection
  • Collections of primitives
    ◦ optimizer
  • Box
    ◦ Add to vec or hashmap to make heterogeneous
    ◦ Dyn
    ◦ Cheaper to move than a struct if the struct is large. So anything large should be heap allocated. (Most collections including String, Vec, HashMap etc already are, so usually not a big deal)
  • Structs containing traits -> box.
    ◦ How ownership prevents generics hell
  • Arc / Rc

In Java you are probably also used to seeing things like:
__
Where the ‘? Extends’ indicates that the value could be any type as long as it implements the interface. This is different from 'T extends’ as there doesn't have to be a single type T. 

So in Java to make a heterogeneous list you might do this:__ That would allow each member of the list to be a different type of _. In Rust you would write it like this __. The “dyn” keyword means that the type is dynamic and won't be available until runtime. 

Notice here a Box is needed because “dyn” implies we don't actually know they type and hence the size which the compiler needs to correctly size the elements of vector.

While that might seem like a fair number of extra characters, it actually opens up a lot of flexibility. If we wanted to call a method appropriate for each member of the list, in Java we would have to implement the Visitor pattern mentioned above __. Where as in Rust we can just write __.

## Iterators
  • Adding close to iterators (not possible in Java) (can pass around in Rust)
  • Fail fast iterators vs compiler
    ◦ Map.entry() example
  • Splitting lists ranges
  • Java streams vs generators
    ◦ Zip pair etc.
    ◦ Composition syntax
    ◦ Things like sum and such that are always on Java’s interface even when they don’t apply. It has mapToInt etc. Because traits can be implemented based on the generic type, in Rust these methods are there when they apply and gone when they don’t. This means they don’t all need weird names based on the type. They can have a single uniform name. What’s more they aren’t tied to concrete types like ‘int’ and ‘long’ they can be tied to “anything implementing add” etc. 
  • For( line : file) example
  • Collect on Java streams vs Rust inferring type from result
    ◦ Single impl of collect as opposed to having a separate one for each type.
    ◦ Collect invers return type or via turbofish

## Closures
  • syntax
  • Map on iterator / option
  • foreach
  • Move
  • Pass functions directly
  • Alias allows equivalent of functional interface in Java.

