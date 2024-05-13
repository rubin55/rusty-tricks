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
  * `str` is usually seen in this form: `&str` (pronounced string slice). It is
    a small type that contains a reference to str data and a length
  * Creating `&str` values avoids a memory allocation
  * `&str` is a borrowed type. In practical terms, this means that `&str` can be
    thought of as read-only data, whereas `String` is read-write
  * `[u8]` is slice of raw bytes
  * `String` is to `Vec<u8>` as `str` is to `[u8]`
  * Array notation: `[f32; 12]` is an array of 12 32-bit floating point values
  * `[u8; 3]` is a different type than `[u8; 4]` - the size of an array matters
  * `Vec<Vec<(usize, String)>>` is a vector of vectors (e.g., `Vec<Vec<T>>`),
    where `T` is a pair of values of type `(usize, String)`. `(usize, String)`
    is a tuple
  * `Vec<T>` performs best when you can provide it with a size hint via
    `Vec::with_ capacity()`
  * Use the `panic!` macro when you need to crash immediately
  * Use the `unreachable!` macro for a code block that should be unreachable
  * Use the `unimplemented!` macro as a placeholder for not-yet implemented code
  * Use the `assert!`, `assert_eq!` and `assert_ne!` macros for tests and
    possibly, pre-conditions for functions (think design-by-contract)
  * `'static` means valid for the entire lifetime of the program
  * `From` trait is for converting between types, called by `?` to convert
    between error types

In Rust, "no value" is represented as `()`, called "unit".
