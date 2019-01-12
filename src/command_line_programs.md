# Command line programs

  • Signals (os)
  • DSL
  • Builder pattern works the same way in both langs, but ownership sometimes helps
  • Clap and glob

Command line applications is another area where rust shines. Normally you might write things in bash but it quickly becomes unreadable after a couple hundred lines of code. Pearl and Python are commonly used and work reasonably well but they have some serious drawbacks in both performance and compatibility. (IE if you wrote and tested you code on version 2.3 on Linux, it might or might not work running in version 2.2 on Windows) You could use Java, Go, or C++ but they involve writing a fair amount of boilerplate and they never really feel very natural for command line application. Where as Rust has high level features and ability to run on Windows, Mac, and Linux without any special work. It can do this by compiling into a single binary file that has no dependencies on any vm, interpreter, or library already being installed on the system. Additionally libraries like `clap` or `quicli` make parsing args easy.
