# Cool Rust Tricks

  * Zero cost abstraction
    * Create a wrapper type like append only vec. (Makes contract clear, removed by optimizer)
      * Can have intoVec which requires ownership. Prevents calling 'externally’
    * Is whitespace
  * Really low memory use
    * Skylight by Tilde went from a 5gb jvm to 50mb in rust
    * 

