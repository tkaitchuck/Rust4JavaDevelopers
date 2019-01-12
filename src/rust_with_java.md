# Using Rust with Java

  • Crossbeam
  • Jni crate
  • Unsafe
    ◦ Don’t do it.
    ◦ Not faster
    ◦ Really unsafe.
    ◦ Better to have tiny parts like split at mut
    ◦ Just because you can doesn't mean you should.
    ◦ Use sanitizers and prop/quickcheck or fuzzer
    ◦ Rust-san
      ▪ Note that rust has __ enabled by default. (As opposed to go)

