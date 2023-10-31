# Changelog - `transform-rs`
This file keeps track of (notable) changes to the `transform`-crate.

Note that this project uses [semantic versioning](https://semver.org). As such, breaking changes are indicated using \[**breaking**\].


## v0.1.1 - 2021-10-31
### Changed
- Slightly better documentation for the trait & iterator, which makes clear that the closure is only called for old elements.

### Fixed
- A bug where a single empty element would produce `None` for the entire iterator preemptively.


## v0.1.0 - 2023-10-31
Initial release!

### Added
- The `TransformIter` iterator.
- The `Transform` trait.
