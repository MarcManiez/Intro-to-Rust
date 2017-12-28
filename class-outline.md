# Introduction

This talk is about Rust, but it’s also going to be about memory management in computer languages to some extent. So for that, we’re going to start with a brief overview of how programs interact with memory.

There are three ways of managing objects in a program:

1. Static values like string literals can be hardcoded in the executable.

2. The stack is an area of memory reserved for data relating to function calls. Each function has what is called a stack frame which lists what goes into a function and what comes out.

The stack is called a stack because it is a stack-type data structure. Each frame or function that returns is popped off the top of the stack. What goes in and out of each stack frame must have a known size.

3. The heap is a less-strictly organized memory area that programs can draw from to store data if its size is unknown at compile time or if it needs to outlive its current stack frame. Objects on the heap often have associated objects called pointers which are a sort of bread crumb trail that points to the location of its corresponding object on the heap. A basic pointer will often have the address in memory (e.g 0x7ffeedb9b4de) along with a name, a capacity (how many bytes have been allocated) and a a size (how many bytes have been used).

Memory in a program is being interacted with constantly, so how a given language decides to interact with memory is character-defining.

Most recent languages, and all scripting languages that I can think of are garbage collected, which means heap allocated memory is routinely and automatically cleared up using various algorithms.

Traditional languages such as C require manual allocation and cleanup of memory using functions such as malloc() and free(), which we’ll get acquainted with in just a bit.

Garbage collection is known for be a very difficult problem to solve, so the control granted by manual memory allocation is powerful if we strive for efficiency. But it is also error prone and leads to two classes of problems that Rust claims to address:

a. Memory leaks

b. Security vulnerabilities (buffer overflow, dangling pointers, etc.) which allow savvy attackers to inject code in poorly written software.

Rust helps dramatically reduce bugs in the first category, and if I’m not mistaken, it claims to entirely remove the possibility for bugs in the second category.

Rust achieves this by using a system called ownership, which will be the main focus of today’s class.

# Ownership

At this point I’m going to quote extensively from a [free online book](https://doc.rust-lang.org/stable/book/second-edition/) that I highly recommend (the first few chapters are particularly eloquent and well-illustrated, so it’s easy to get into!).

The rules of ownership are simple at first glance:

1. You don’t talk about ownership.

End of class. See you next week...!

1. Each value in Rust has a variable that’s called its _owner_.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

[Basic ownership examples](rust-examples/basic-ownership/src/main.rs)