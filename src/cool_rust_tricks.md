# Cool Rust Tricks
Rust is built on the idea of "Zero Cost Abstractions".
This means that unlike in Java, [if you add a wrapping layer or extra traits, this won't add any overhead](https://www.youtube.com/watch?v=Sn3JklPAVLk)

So you can define: 
```rust
# #![allow(dead_code)]
# #![allow(unused)]
struct AppendOnlyVec<T> {
    inner: Vec<T>,
}
impl<T> AppendOnlyVec<T> {
    fn new(v: Vec<T>) -> AppendOnlyVec<T> {
        AppendOnlyVec{ inner: v }
    }
    fn push(&mut self, new_element: T) {
        self.inner.push(new_element);
    }
    fn into_vec(self) -> Vec<T> {
        self.inner
    }
}
# fn main() {}
```
This type wraps a vec (by taking ownership of it so that it cannot be modified elsewhere).
At compile time this struct will get inlined, as will all of the method implementations.
After complication the only thing left is primitives and gotos. Exactly as though everything were written in hand optimized assembly.

So the above example and other similar abstraction can be added to clearify code without any overhead.

Compared to Java this model allows Rust to have very low memory use.
For example when [Tilde started using Rust](https://www.rust-lang.org/static/pdfs/Rust-Tilde-Whitepaper.pdf) 
their memory use decreased by 99%.


