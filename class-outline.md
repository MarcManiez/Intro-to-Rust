# Introduction

This talk is about Rust, but it’s also going to be about memory management in computer languages to some extent. So for that, we’re going to start with a brief overview of how programs interact with memory.

There are three ways of managing objects in a program:

1. Static values like string literals can be hardcoded in the executable.

2. The stack is an area of memory reserved for data relating to function calls. Each function has what is called a stack frame which lists what goes into a function and what comes out.

The stack is called a stack because it is a stack-type data structure. Calling a function puts a frame on the stack and each function that returns has its stack frame is popped off the top. What goes in and out of each stack frame must have a known size.

3. The heap is a less-strictly organized memory area that programs can draw from to store data if its size is unknown at compile time or if it needs to outlive its current stack frame. Objects on the heap often have associated objects called pointers which are a sort of bread crumb trail that points to the location of its corresponding object on the heap. A basic pointer will often have the address in memory (e.g 0x7ffeedb9b4de) along with a name, a capacity (how many bytes have been allocated) and a size (how many bytes have been used).

Memory in a program is being interacted with constantly, so how a given language decides to deal with memory is character-defining.

Most recent languages, and all scripting languages that I can think of are garbage collected, which means heap allocated memory is routinely and automatically cleared up using various algorithms.

Traditional languages such as C require manual allocation and cleanup of memory using functions such as malloc() and free(), which we’ll get acquainted with in just a bit.

Garbage collection is a notoriously difficult problem to solve, so the control granted by manual memory allocation is powerful if we strive for efficiency. But it is also error prone and leads to two classes of problems that Rust claims to address:

1. Memory leaks.

2. Security vulnerabilities (buffer overflow, dangling pointers, etc.) which allow savvy attackers to inject code in poorly written software.

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

[Memory leak example in C](c-examples/memory-leak/memory_leak.c)

But as rule #2 reminds us, it’s not quite as simple as dropping any variable when it gets out of scope. If we did that, and that alone, we might end up double freeing.

[Double free example in C](c-examples/double-free/double_free.c)

[Double free counter-example in Rust](rust-examples/double-free/src/main.rs)

As the second example illustrates, Rust has a concept of moving ownership between variables. This allows us to fulfill requirement #2: that each heap-allocated object only have one owner. It also avoids double frees.

## References and borrowing

How does ownership affect function calls? What needs to happen when we want to pass a value into a function?

[Interactive example](rust-examples/references/src/main.rs)

## Mutability

By default, variable declarations in Rust are immutable. Most variables in programming end up not being mutated so setting this as the default makes sense. It also makes mutable variables very explicit.

[Here’s an example of mutability in action](rust-examples/mutability/src/main.rs)

## Advanced borrowing examples

Now that we know the fundamentals of ownership in Rust, let’s take some more side-by-side comparisons with C to see how differently the languages handle similar situations.

[Mutable pointers in Rust](rust-examples/mutable-pointers/src/main.rs)

[Mutable pointers in C](c-examples/mutable-pointers/mutable_pointers.c)

# Lifetimes

Lifetimes are involved whenever a reference is made and a value is borrowed. It’s the feature of Rust that ensures that you can’t access invalid parts of memory.

Lifetimes help ensure that the value being borrowed has a lifetime that is >= the lifetime of the reference.

[Example of a dangling pointer in C](c-examples/dangling-pointer/dangling_pointer.c)

[Example of a dangling pointer in Rust](rust-examples/dangling-pointer/src/main.rs)

As we can see in the example, if a reference lives after the value it borrows has been dropped, it points to nothing, which is dangerous.

Often times, lifetimes can be elided and inferred, simply because only one reference is present in a given scope.

```Rust
Fn func(reference: &String) -> reference: &String {
    reference
}

fn main() {
    string = String::from("String");
    func(&string);
}
```

But in more complex scenarios, the compiler asks us to specify lifetimes. We’ll see why it’s costly, or impossible for the compiler to derive this information itself. Let's consider a simple function that seeks to return the longest of two string.

[Interactive lifetimes example](rust-examples/lifetimes/src/main.rs)

## Static lifetime

There is one special lifetime: `'static`. This means that the reference should be made to live for the entirety of the program’s life.

## Lifetime subtyping

We can declare a hierarchy between different lifetimes. For example:

```Rust
struct SubStruct<'s> (&'s str);

struct Struct<'c, 's: 'c> {
    sub_struct: &'c SubStruct<'s>,
}
```

This means that `'s` has a lifetime that is >= the lifetime of `'c`.

# Enumerables & Pattern Matching

Pattern matching and enumerables are not unique to Rust, but they are rare enough in computer languages that they are worth pointing out. When combined, these two features open a whole new world of control flow, making `if`s and `else`s the exception rather than the norm.

## Enumerables

Enumerables are referred to as `enums` for short. You can think of them as a category which lists out different variants within that category. Here's an example:

```Rust
enum Identifier<'a> {
    Email(&'a str),
    Telephone(&'a str),
    SSN(usize),
    Username(&'a str),
}

let ssn = Identifier::SSN(0000000000);
```

You can optionally store some data inside an enum's variant. In this case, we stored our social security number. But why is this useful? To understand, lets look at another example and introduce another type of enum, `Option`. Imagine that you're trying to fetch user information from a database:

```Rust
struct User {
    primary_identifier: Identifier, // note that we can use enums as types
    email: Identifier::Email,
    telephone: Identifier::Telephone,
    ssn: Identifier::SSN,
    username: Identifier::Username,
    other_valuable_information: String,
}

fn find_user(identifier: Identifier) -> User {
    // some code to retrieve a user from the database
    // using the identifier argument
}
```

How do we handle the failure scenario? In most languages, we would be content by returning the user, or `null` if we didn't find anything. In Rust, that's a problem for two reasons:

* Rust is strongly typed. If `find_user`'s signature tells us we're returning a user, we must return a user in all cases. The problem here is that we can't guarantee our database will cooperate 100% of the time.
* Rust has no concept of `null`.

This how Rust addresses this situation:

```Rust
enum Option<T> {
    Some(T),
    None,
}

fn find_user(identifier: Identifier) -> Option<User> {
    // some code to retrieve the user from the database
}
```

`Option` is such a widespread enum that it is part of Rust standard library.

This pattern has a few advantages:

* It lets `find_user` be concerned with only its primary duty: finding a user, and it lets the caller decide what to do with the result.
* It removes the ambiguity of coercing everything to a boolean, which many languages do inconsistently.
* It opens up some very powerful control flow tools.

## Pattern Matching

Pattern matching is the ability to branch the flow of our program depending on an enumerable. The standard way to branch off of multiple potential enum variants is to use the `match` keyword. Let's continue our example to illustrate:

```Rust
enum Identifier<'a> {
    Email(&'a str),
    Telephone(&'a str),
    SSN(usize),
    Username(&'a str),
}

fn find_user(identifier: Identifier) -> Option<User> {
    match identifier {
        Email(email) => search_user_table_by_email(email),
        Telephone(phone) => search_user_table_by_telephone(phone),
        _ => None, // we got lazy at the end and used a catch-all, because we can!
    } // returns implicitly
}

fn search_user_table_by_email(email: &str) -> Option<User> { /* some code */ };
fn search_user_table_by_telephone(email: &str) -> Option<User> { /* some code */ };
```

The call site of `find_user` might look something like this:

```Rust
let email = Idenfier::Email("dev@ops.org");

match find_user(email) {
    Some(user) => /* Do something with the `user` variable */,
    None => /* Handle the failure case */,
}
```

If all we care about is handling one scenario, Rust gives us this syntax:

```Rust
let email = Idenfier::Email("dev@ops.org");

if let Some(user) = find_user(email) {
    // do something with the `user` variable
}

// don't handle failure case because yolo
```
