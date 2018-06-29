# Benchmark `Option::replace_with` RFC

This benchmark compares three implementations of the following idea ([more details in Rust RFC #2940](https://github.com/rust-lang/rfcs/pull/2490)):

```rust
let mut some_option: Option<i32> = Some(123);

some_option = consume_option_i32_and_produce_option_i32(some_option.take());
```

This straightforward implementation turns out to be non-optimal and thus some `unsafe` + `mem::forget` produces faster code and thus it is proposed to add a special `Option::replace_with` method.

In this benchmark compares:

1. Naive `Option::take` + reassignment implementation
2. Ad-hoc `mem::swap` + `mem::forget` implementation
3. Proposed `Option::replace_with` implementation

To run the benchmark, just do `cargo run --release`.

Here is the output on my Core i7-4710HQ laptop (Arch Linux, x64, 4.17.2 kernel):

```
Replace Option with a new value computed from an old value/naive assignment
                        time:   [31.159 ns 31.259 ns 31.368 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe


Replace Option with a new value computed from an old value/mem::swap + mem::f...
                        time:   [26.729 ns 26.836 ns 26.956 ns]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe


Replace Option with a new value computed from an old value/Option::replace_with
                        time:   [26.742 ns 26.816 ns 26.903 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```

Thus, naive implementation is about 14% slower (31 ns VS 26 ns) than the proposed implementation.

Here is a comparison of the produced assembly code: https://godbolt.org/g/6Cukig (naive implementation is on the left, and optimized implementation is on the right).
