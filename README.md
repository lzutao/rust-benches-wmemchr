This crate is used to benchmark the performance difference between
Rust's `iter::position` and C's `wmemchr` on Windows target.

## For MSVC

Run:

```bash
RUSTFLAGS='-C target-feature=+crt-static' cargo bench --target x86_64-pc-windows-msvc
```
