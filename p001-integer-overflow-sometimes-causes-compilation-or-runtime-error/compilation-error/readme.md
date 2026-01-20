`cargo build` and `cargo build --release` error.

Relevant messages:

> attempt to compute `u8::MAX + 1_u8`, which would overflow

> = note: `#[deny(arithmetic_overflow)]` on by default

| command                 | result |
| ----------------------- | ------ |
| `cargo build`           | error  |
| `cargo build --release` | error  |
| `cargo run`             |        |
| `cargo run --release`   |        |
