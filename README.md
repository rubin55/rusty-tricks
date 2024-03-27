# rusty-tricks

A collection of various Rust cargo projects, probably from various books and
courses, in particular, The Rust Programming Language, Programming Rust, Rust in
Action and Programming with Rust.

Each sub-directory is a cargo project. See `README.md` there for any project
specific details.

## Some notes

Rust is an expression-based language; almost everything returns something.
There are a few exceptions:

  * Expressions delimited by `;`
  * Binding a name to a value with `=`
  * Type declarations, which include `fn`, `struct` and `enum`

In Rust, "no value" is represented as `()`, called "unit".


