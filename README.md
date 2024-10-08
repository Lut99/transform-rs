# transform-rs
A small Rust library that defined the `TransformIter`-iterator, which can map an element in a tuple to zero or more elements of a potentially different type.


## Installation
To install the `transform`-crate, simply add it to your `Cargo.toml` file:
```toml
transform = { git = "https://github.com/Lut99/transform-rs" }
```

You can also commit yourself to a particular version by using the `tag`-key.
```toml
transform = { git = "https://github.com/Lut99/transform-rs", tag = "v0.1.0" }
```


## Usage
To use this library, first add the `Transform`-trait to your current scope:
```rust
use transform::Transform as _;
// Or, if preferred:
use transform::prelude::*;

// ...
```
Next, you can call `transform()` on iterator functions to do things like content-based expansion:
```rust
let numbers = vec![1, 2, 3, 4, 5];
let numbers: Vec<i32> = numbers.into_iter().transform(|num| vec![num; num as usize]).collect();
assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
```


## Features
This crate does not have any features.


## License
This project is now licensed under Apache 2.0. See [`LICENSE`](./LICENSE) for more information.


## Contributing
If you want to contribute to this create, welcome! Feel free to [raise an issue](https://github.com/Lut99/transform-rs/issues) or [create a pull request](https://github.com/Lut99/transform-rs/pulls).

Note, however, that this is a hobby project. As such, I might not adopt all suggestions, no matter how good they are ;)
