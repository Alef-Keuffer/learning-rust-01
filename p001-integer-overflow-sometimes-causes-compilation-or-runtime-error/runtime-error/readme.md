`cargo build` and `cargo build --release` succeed without producing errors
related to integer overflow.

`cargo run` failed with relevant message:

```
thread 'main' (72594) panicked at src/main.rs:3:20:
attempt to add with overflow
```

`cargo run --release` succeeds.
