# Why Rust
Rust is designed to be a safe, productive, and performant general-purpose programming language.

In that space there is a lot of competition. New language since c has ever really taken over that space. C++, Python, Java, and C# have each, carved off a piece of that space. But none of them have ever really been able to be the best in all of the different niches.

To even carve off a portion of this space is quite a challenge because any new language has to be better than the best language out there for a given usecase. It can't be just a little bit better either. It has to be a lot better, because there are huge switching costs. Libraries need to be rewritten. People need to be retrained. To gain traction a new language needs to be a lot better.

Rust is a lot better.

Rust is still very young but it's starting to compete with other languages on their own terms. Some have argued that may soon be better for web development than Javascript. And in many ways it is already a better C++ than C++. But, as I will argue in this book: Rust is a better Java than Java.

I decided to write this book because a lot of the Rust documentation was written in a way that was geared towards people coming from C++. If you know about pointers, memory layout, and RIIA are second nature to you, then these explanations make sense. I wanted to go in a different direction describe Rust from the perspective of Java. As such, this book won’t mention any of those things, and will describe and define how Rust work from the perspective of a Java developer. This book is first and foremost targeted at developers who already know and have experience in Java and are looking to learn Rust. If this is not your situation you might still get a fair amount out of this book but it is not my goal. 

Throughout, the book will provide side-by-side comparisons of Java and Rust code that is identical in functionality. Because the goal is to facilitate comparison between the languages sometimes the examples are not idiomatic Rust or idiomatic Java. Rather they are written to make the comparison clear and direct.

Throughout the book we’ll have running commentary from two characters:

<table width="100%">
<tr>
<td> 

![Safety monitor](images/borrow.png)
</td>
<td width="80%">

> *“Hi, I make sure your program is correct, and won’t fail at runtime. A lot of people call me ‘the borrow checker’ because that’s a big part of my job. But I do more than that. I enforce style guidelines, memory safety, thread safety, type safety and more. I’ll act as a guide and alert you if you do anything that could go wrong.”*
</td>
<tr>
<td>

![Optimizer](images/professor.png)
</td>
<td width="80%">

> *“Hi, I focus on making everything you write go as fast as possible. My goal is to allow you to write code in the safest, most readable, way possible so you don’t make mistakes. Then once she has approved it, I’ll take over and remove all of those abstractions so they don’t cost you a thing. Then I fine tune everything to be as fast as if you had written it all in assembler by hand.”*
</td>
</tr>
</table>
