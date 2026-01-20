A few of the rules I think are worth noting down to keep in mind

- each value in Rust has an owner
- there can only be one owner at a time
- when the owners goes out of scope, the value will be dropped

- given pointer ´p1´, doing ´let p2 = p1´ invalidates ´p1´ by doing a _move_.
  ´s1´ was _moved_ into ´s2´.

- on Rust, types that can be stored on the stack implement the Copy trait
- a type that implements the Drop trait can't implement the Drop trait
- we can use a value without transferring ownership: references
- a reference is like a pointer: in that it's an address we can follow to access
  data stored at that address
- that data is owned by another variable
- the action of creating a reference is called _borrowing_
- to modify a value you need a _mutable reference_
- if you have a mutable reference to a value, you can have no other references
  to that value

- A foundational goal of Rust is to ensure that your programs never have
  undefined behavior.
- A secondary goal of Rust is to prevent undefined behavior at compile-time
  instead of run-time. This goal has two motivations:

- At any given time, you can have either one mutable reference or any number of
  immutable references.
- References must always be valid.

- References Are Non-Owning Pointers

- Pointer Safety Principle: data should never be aliased and mutated at the same
  time.

The core idea behind the borrow checker is that variables have three kinds of
permissions on their data:

- Read (R): data can be copied to another location.
- Write (W): data can be mutated.
- Own (O): data can be moved or dropped.

By default, a variable has read/own permissions (RO) on its data. If a variable
is annotated with let mut, then it also has the write permission (W). The key
idea is that references can temporarily remove these permissions.

- immutable references are also called shared references
- mutable references are also called unique references
- data must outlive any references to it

- if a value does not own heap data, then it can be copied without a move.

## Useful Tools

- [Aquascope]

## On Aquascope

three programs:

```rs
let sT1: String = String::from("Hello World");
let sstr1: &str = &sT1;
let v1: Vec<&str> = vec![sstr1];
let s1: &str = v1[0];
```

```rs
let sT2: String = String::from("Hello World");
let sstr2: &str = &sT2;
let v2: Vec<&String> = vec![&sT2];
let s2: &String = v2[0];
```

```rs
let sT3: String = String::from("Hello World");
let sstr3: &str = &sT3;
let v3: Vec<String> = vec![String::from("Hello World")];
let s3: &String = &v3[0];
```

This causes an error (pointer out of bounds) on Aquascope:

```rs
let sT2: String = String::from("Hello World");
let sT21: String = String::from("Hello World");
let sstr2: &str = &sT2;
let sstr21: &str = &sT21;
let v2: Vec<&String> = vec![&sT2, &sT21];
let s2: &String = v2[0];
```

## Question

Question 4 of quiz at
https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#fr-boxed-data-structures-1
says

Question 4

Say we have a function that moves a box, like this:

```rs
fn move_a_box(b: Box<i32>) {
  // This space intentionally left blank
}
```

Below are four snippets which are rejected by the Rust compiler. Imagine that
Rust instead allowed these snippets to compile and run. Select each snippet that
would cause undefined behavior, or select "None of these snippets" if none of
these snippets would cause undefined behavior.

```rs
let b = Box::new(0);
let b2 = b;
move_a_box(b);
```

None of these snippets

```rs
let b = Box::new(0);
move_a_box(b);
println!("{}", b);
```

```rs
let b = Box::new(0);
move_a_box(b);
let b2 = b;
```

```rs
let b = Box::new(0);
let b2 = b;
println!("{}", b);
move_a_box(b2);
```

The correct answer is:

```rs
let b = Box::new(0);
let b2 = b;
move_a_box(b);
```

```rs
let b = Box::new(0);
move_a_box(b);
println!("{}", b);
```

```rs
let b = Box::new(0);
move_a_box(b);
let b2 = b;
```

Context: The key idea is that when a box is passed to move_a_box, its memory is
deallocated after move_a_box ends. Therefore:

Reading b via println after move_a_box is undefined behavior, as it reads freed
memory.

Giving b a second owner is undefined behavior, as it would cause Rust to free
the box a second time on behalf of b2. It doesn't matter whether the let b2 = b
binding happens before or after move_a_box.

However, doing let b2 = b and then println is not undefined behavior. Although b
is moved, its data is not deallocated until move_a_box is called at the end.
Therefore this program is technically safe, although still rejected by Rust.

_My question_ is, how is

```rs
let b = Box::new(0);
let b2 = b;
println!("{}", b);
move_a_box(b2);
```

not also undefined behavior? Doesn't it also give b a second owner by causing it
to be freed twice?

## Question

I feel like the answer to the last question of the last quiz of
https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html is wrong.

The question is

Consider this Rust function that pushes a number onto the end of a vector, and
then removes and returns the number from the front of the vector:

```rs
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}
```

Normally, if you try to compile this function, the compiler returns the
following error:

```
error[E0596]: cannot borrow `*v` as mutable, as it is behind a `&` reference
 --> test.rs:2:5
  |
1 | fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
  |                     --------- help: consider changing this to be a mutable reference: `&mut Vec<i32>`
2 |     v.push(n);
  |     ^^^^^^^^^ `v` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Assume that the compiler did NOT reject this function. Select each (if any) of
the following programs that could possibly cause undefined behavior if executed.
If none of these programs could cause undefined behavior, then check "None of
these programs" .

```rs
let v = vec![1, 2, 3];
let v2 = &v;
give_and_take(&v, 4);
println!("{}", v2[0]);
```

```rs
let v = vec![1, 2, 3];
let n = &v[0];
give_and_take(&v, 4);
println!("{}", n);
```

```rs
let v = vec![1, 2, 3];
let n = &v[0];
let k = give_and_take(&v, 4);
println!("{}", k);
```

None of these programs

Context: As we saw earlier in the section, `v.push(n)` can cause `v` to
reallocate its internal contents, invalidating any references to the elements of
`v` on the heap. Therefore calling `give_and_take(&v, 4)` will cause
previously-created element references to point to invalid memory. The two
programs that bind `let n = &v[0]` before `give_and_take` are candidates for
undefined behavior. `let v2 = &v` is not a candidate because a reference to the
container `v` is not actually invalidated by mutating `v`.

The program that does `println!("{}", n)` will cause undefined behavior by
reading the invalid memory. The program that does `println!("{}", k)` will not
cause undefined behavior, because it does not use the invalidated reference.

My take again: I do think a reference to a container can be invalidated by
mutating it. The vector can grow in size beyond its original capacity and
reallocation may require to move the location of the container in memory.
Therefore, the reference would be invalidated.

## Question take_while Ch 21 multithreaded web server

Taking into account this content at

```rs
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
```

Why does the code below only print on request? Furthermore, it only seems to
print after I make the first request, i.e., it's as if it dropped the first
request and thereafter it printed the requests.

```rs
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(Result::unwrap)
        .take_while(|line| true)
        .collect();

    println!("Request: {http_request:#?}");
}
```

While this code

```rs
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(Result::unwrap)
        .take_while(|_| false)
        .collect();

    println!("Request: {http_request:#?}");
}
```

prints this:

```
Request: []
Request: []
Request: []
Request: []
Request: []
Request: []
Request: []
Request: []
Request: []
Request: []
```

?

`take_while` is necessary because otherwise we have an infinite iterator waiting
for new lines from the buffer.

Without `take_while`, when I refresh the page the browser probably sends "End of
File" (EOF) signal to lines() breaking the loop. We're probably accumulating
streams in the first case and always waiting for lines() stuck on the loop.

<!--References-->

[aquascope]: https://cel.cs.brown.edu/aquascope/ "Aquascope"
