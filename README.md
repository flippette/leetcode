# leetcode

LeetCode solutions in Rust.

## structure

problems are categorized according to their difficulty and number as seen on
LeetCode, and are located in their respective submodules, e.g. Two Sum is
`easy::lc1`, Regular Expression Matching is located in `hard::lc10`, etc.

each problem's module also include the base case tests from LeetCode, additional
failed tests during grading, as well as other test cases to satisfy
`cargo-tarpaulin`.

## dependencies

solutions only depend on crates supplied by the LeetCode grader (at the time of
writing: `itertools@0.14`, `rand@0.8`, and `regex@1`).

## unsafe code

this crate is `#![deny(unsafe_code)]` and
`#![forbid(clippy::missing_safety_doc)]`, so unsafe code must be documented and
explicitly allowed.

## MSRV

the MSRV this crate supports is the Rust's latest stable version (at the time
of writing, this is Rust 1.88.0).
