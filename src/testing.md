# Testing

  * Assert and derived macros (dbc and hoare)
    * Used as preconditions and for tests

Rust has a strong culture of testing. Testing is built into cargo. They are run by invoking 'cargo test’. Tests are declared like this __. This is very similar to Junit in Java. Assertions can be made like __ or __.

The way interfaces work in Rust make it easy to mock out components. For example __.

Cargo can also generate code coverage reports via the __ command. These are compatible with _ and so you can visualize them in __.

Rust also supports randomized testing. Two popular libraries for this are _ and _. They work as follows __ or __ <need to pick one>

Cargo also has built in support for microbenchmarking. This is done by writing a test like __. The _ does all the timing for you. All you have to do is implement the function to be benchmarked. You can run benchmarks via _. This shows output and allows you to track changes across versions by __.

  * Tools for profiling: perf
Visible for testing and Mocks
  * Mocking example
  * Yup-hyper-mock

  