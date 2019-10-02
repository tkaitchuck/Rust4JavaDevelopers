# Collections
  * Statically constructed collection
  * Collections of primitives
    * optimizer
  * Box
    * Add to vec or hashmap to make heterogeneous
    * Dyn
    * Cheaper to move than a struct if the struct is large. So anything large should be heap allocated. (Most collections including String, Vec, HashMap etc already are, so usually not a big deal)
  * Structs containing traits -> box.
    * How ownership prevents generics hell
  * Arc / Rc

## Common collections and their equivlents

|Java Type   | Rust Type | Notes |
|------------|-----------|-------|
|ArrayDeque  |VecDeque   |       |
|ArrayList   |Vec        |       |
|BitSet      |FixedBitSet|External crate. (Not in std lib)|
|HashMap / HashSet     |HashMap / HashSet    |       |
|LinkedHashMap / LinkedHashSet|IndexMap / IndexSet  |External crate. (Not in std lib)|
|LinkedList  |LinkedList |Use discoruaged|
|PriorityQueue|BinaryHeap|       |
|TreeMap     |BTreeMap   |Uses BTreee rather than Binary tree (Much faster)|
|TreeSet     |BTreeSet   |Uses BTreee rather than Binary tree (Much faster)|


## Iterators
Rust's iterators are a bit different from Java's:

```java
public interface Iterator<E> {
      /**
        * Returns {@code true} if the iteration has more elements.
        * (In other words, returns {@code true} if {@link #next} would
        * return an element rather than throwing an exception.)
        *
        * @return {@code true} if the iteration has more elements
        */
       boolean hasNext();
   
       /**
        * Returns the next element in the iteration.
        *
        * @return the next element in the iteration
        * @throws NoSuchElementException if the iteration has no more elements
        */
        E next();
}
```
```rust ,skt-default
pub trait Iterator {
    type Item;
    /// Advances the iterator and returns the next value.
    ///
    /// Returns [`None`] when iteration is finished. Individual iterator
    /// implementations may choose to resume iteration, and so calling `next()`
    /// again may or may not eventually start returning [`Some(Item)`] again at some
    /// point.
    ///
    /// [`None`]: ../../std/option/enum.Option.html#variant.None
    /// [`Some(Item)`]: ../../std/option/enum.Option.html#variant.Some
    fn next(&mut self) -> Option<Self::Item>;
}
```

This is both more continent for to implement, because the implementation does not need to know in advance if there is another item.
`Iterator` is also a very good example of the power of Rust's trait system. Implementing `Iterator` only requires writing one method
but it provides callers with over 60. This allows you to do things like:

```rust ,skt-main
# use std::collections::HashMap;
# use std::vec;
let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let map: HashMap<i32, i32> = (0..8).filter(|&x| x%2 == 0).map(|x| (x, x * x)).collect();
```


  * Adding close to iterators (not possible in Java) (can pass around in Rust)
  * Fail fast iterators vs compiler
    * Map.entry() example
  * Splitting lists ranges
  * Java streams vs generators
    * Zip pair etc.
    * Composition syntax
    * Things like sum and such that are always on Java’s interface even when they don’t apply. It has mapToInt etc. Because traits can be implemented based on the generic type, in Rust these methods are there when they apply and gone when they don’t. This means they don’t all need weird names based on the type. They can have a single uniform name. What’s more they aren’t tied to concrete types like ‘int’ and ‘long’ they can be tied to “anything implementing add” etc. 
  * For( line : file) example
  * Collect on Java streams vs Rust inferring type from result
    * Single impl of collect as opposed to having a separate one for each type.
    * Collect invers return type or via turbofish

## Closures
  * syntax
  * Map on iterator / option
  * foreach
  * Move
  * Pass functions directly
  * Alias allows equivalent of functional interface in Java.

