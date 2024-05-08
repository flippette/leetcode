# `leetcode`

"my" solutions to leetcode problems.

## notes

i have changed the types on function parameters, for example:

- `Vec<i32>` -> `impl AsRef<[i32]>`
- `String` -> `impl AsRef<str>`
- (pseudocode) `fn(Vec<String>)` -> `fn<S: AsRef<str>, V: AsRef<[S]>>(V)`

this is done for ease of testing, as these `impl AsRef` signatures accept more
types than the default templates, while still remaining perfectly amenable to
copy-pasting to leetcode directly.
