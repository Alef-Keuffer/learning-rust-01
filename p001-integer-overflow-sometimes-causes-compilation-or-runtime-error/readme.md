## Overview

See

- [StackOverflow question][stack-question]
- [Integer Overflow][iover]
- [Change the semantics of the built-in fixed-size integer types from being
  defined as wrapping around on overflow to it being considered a program
  error][rfcs-0560]

We'll count lines starting from `let num: u8 = 255;`.

Let all the programs have this in common:

```rs
fn main() {
    let num: u8 = 255;
    let num2: u8 = num + 1;
}
```

This case (with 3rd line empty), fails to compile (`cargo build` and
`cargo build --release`).

If the 3rd line is `&num;`, then `cargo build` and `cargo build --release` work.
`cargo run` will panic with `attempt to add with overflow` while
`cargo run --release` will succeed.

## Question regarding Integer Overflow

The documentation says

> "relying on integer overflow's wrapping behavior is considered an error"

Then, it suggests four methods to handle the possibility of overflow:

- `wrapping_*`: modular addition, e.g., see [wrapping_add][rdocs-i8-wrap]
- `checked_*`: e.g., see [checked_add][rdocs-i8-check]
- `overflowing_*`: e.g., see [overflowing_add][rdocs-i8-over]
- `saturating_*`: see [Saturation Arithmetic][sat-arith] and [Struct
  Saturating][rdocs-sat]

My question is, why don't the default arithmetic operations use one of those
methods?

The documentation says

> When you’re compiling in release mode with the `--release` flag, Rust does
> _not_ include checks for integer overflow that cause panics. Instead, if
> overflow occurs, Rust performs _two’s complement wrapping_.

But I couldn't observe this behavior. In my tests, either both failed when
compiling or both succeeded. The difference was that running the release would
not panic.

<!-- References -->

[iover]:
  https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
  "Integer Overflow"
[stack-question]:
  https://stackoverflow.com/questions/75939406/in-rust-why-does-integer-overflow-sometimes-cause-compilation-error-or-runtime
  "In Rust, Why does integer overflow sometimes cause compilation error or runtime error?"
[sat-arith]:
  https://en.wikipedia.org/wiki/Saturation_arithmetic
  "Saturation arithmetic"
[rdocs-sat]:
  https://doc.rust-lang.org/std/num/struct.Saturating.html
  "Struct Saturating"
[rdocs-i8-over]:
  https://doc.rust-lang.org/std/primitive.i8.html#method.overflowing_add
  "overflowing_add"
[rdocs-i8-check]:
  https://doc.rust-lang.org/std/primitive.i8.html#method.checked_add
  "checked_add"
[rdocs-i8-wrap]:
  https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_add
  "wrapping_add"
[rfcs-0560]:
  https://rust-lang.github.io/rfcs/0560-integer-overflow.html
  "Integer Overflow"
