# Writing Mactos

  • Vs reflection

In Rust you will find yourself using macros in a lot of the same places you would use reflection in Java. Which is to say in a normal program, almost never. But they are often a core component of any sort of framework or library that automates writing a bunch of boilerplate code.

We've already seen simple macros like 'println!’, 'format!’ as well as some more complicated ones like Serde’s serialize/deserialize. Now we're going to get into how they work.

Macros work by running code at compilation time. This is done by invoking the macro i to the

Macros can even be used to write other macros. Because writing macros is complicated there is a simple macro defmac! That lets you define a macro with a very simple syntax. You can write __ to define a macro that will __. Compared to this Java code __ . This is all run at compile time and ____

