//  LIB.rs
//    by Lut99
//
//  Created:
//    31 Oct 2023, 16:07:42
//  Last edited:
//    31 Oct 2023, 16:34:52
//  Auto updated?
//    Yes
//
//  Description:
//!   A small Rust library that defined the [`Transform`] iterator, which
//!   can map an element in a tuple to zero or more elements of a
//!   potentially different type.
//!   
//!   # Installation
//!   To install the `transform`-crate, simply add it to your `Cargo.toml` file:
//!   ```toml
//!   transform = { git = "https://github.com/Lut99/transform-rs" }
//!   ```
//!   
//!   You can also commit yourself to a particular version by using the `tag`-key.
//!   ```toml
//!   transform = { git = "https://github.com/Lut99/transform-rs", tag = "v0.1.0" }
//!   ```
//!   
//!   
//!   # Usage
//!   To use this library, first add the `Transform`-trait to your current scope:
//!   ```rust
//!   use transform::Transform as _;
//!   // Or, if preferred:
//!   use transform::prelude::*;
//!   
//!   // ...
//!   ```
//!   Next, you can call `transform()` on iterator functions to do things like content-based expansion:
//!   ```rust
//!   let numbers = vec![1, 2, 3, 4, 5];
//!   let numbers: Vec<i32> = numbers.into_iter().transform(|num| vec![num; num as usize]).collect();
//!   assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
//!   ```
//!   
//!   
//!   # Features
//!   This crate does not have any features.
//!   
//!   
//!   # License
//!   This project is licensed under GPLv3. See [`LICENSE`](./LICENSE) for more information.
//!   
//!   
//!   # Contributing
//!   If you want to contribute to this create, welcome! Feel free to [raise an issue](https://github.com/Lut99/transform-rs/issues) or [create a pull request](https://github.com/Lut99/transform-rs/pulls).
//!   
//!   Note, however, that this is a hobby project. As such, I might not adopt all suggestions, no matter how good they are ;)
//

// Defines a convenient place for importing traits
pub mod prelude {
    pub use super::Transform;
}


/***** LIBRARY *****/
/// An iterator that can map each element in the underlying iterator to zero or more elements of another iterator.
///
/// # Examples
/// ```rust
/// use transform::Transform as _;
///
/// // Transform can be used for content-based expansion
/// let numbers = vec![1, 2, 3, 4, 5];
/// let numbers: Vec<i32> = numbers.into_iter().transform(|num| vec![num; num as usize]).collect();
/// assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
///
/// // We can also clone elements while doing so
/// let numbers = vec![1, 2, 3, 4, 5];
/// let numbers: Vec<i32> = numbers.iter().transform(|num| vec![*num; *num as usize]).collect();
/// assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
///
/// // Or change data types altogether
/// let chars = vec!['a', 'b', 'c'];
/// let padded_chars: Vec<u8> = chars.into_iter().transform(|c| [0, 0, 0, c as u8]).collect();
/// assert_eq!(padded_chars, vec![0, 0, 0, 97, 0, 0, 0, 98, 0, 0, 0, 99]);
/// ```
pub struct TransformIter<I, F, R>
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: IntoIterator,
{
    /// The underlying iterator to apply transform over.
    iter: I,
    /// The closure to apply on elements of `I`
    func: F,
    /// The buffer of elements to flush first.
    buf:  Option<R::IntoIter>,
}
impl<I, F, R> Iterator for TransformIter<I, F, R>
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
    R: IntoIterator,
{
    type Item = R::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Attempt to flush the buffer first
        if let Some(next) = self.buf.as_mut().map(|buf| buf.next()).flatten() {
            return Some(next);
        }

        // Else, attempt to get the next element from the iterator
        let next: I::Item = match self.iter.next() {
            Some(next) => next,
            None => return None,
        };

        // If we haven't got anything in the buffer but we do have an item in the iterator, then get the next buffer
        self.buf = Some((self.func)(next).into_iter());
        self.buf.as_mut().unwrap().next()
    }
}



/// A trait that adds the [`transform()`](Transform::transform())-function to all [`Iterator`]s.
///
/// # Examples
/// ```rust
/// use transform::Transform as _;
///
/// // Transform can be used for content-based expansion
/// let numbers = vec![1, 2, 3, 4, 5];
/// let numbers: Vec<i32> = numbers.into_iter().transform(|num| vec![num; num as usize]).collect();
/// assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
///
/// // We can also clone elements while doing so
/// let numbers = vec![1, 2, 3, 4, 5];
/// let numbers: Vec<i32> = numbers.iter().transform(|num| vec![*num; *num as usize]).collect();
/// assert_eq!(numbers, vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);
///
/// // Or change data types altogether
/// let chars = vec!['a', 'b', 'c'];
/// let padded_chars: Vec<u8> = chars.into_iter().transform(|c| [0, 0, 0, c as u8]).collect();
/// assert_eq!(padded_chars, vec![0, 0, 0, 97, 0, 0, 0, 98, 0, 0, 0, 99]);
/// ```
pub trait Transform: Iterator {
    /// Transforms each element in the iterator into zero or more elements of a (potentially) different type.
    ///
    /// # Arguments
    /// - `predicate`: A closure that transforms each element in the iterator into an iterator that produces the zero-or-more elements of another type.
    ///
    /// # Returns
    /// A new [`TransformIter`] that can do as advertised.
    fn transform<F, R>(self, predicate: F) -> TransformIter<Self, F, R>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> R,
        R: IntoIterator,
    {
        TransformIter { iter: self, func: predicate, buf: None }
    }
}
impl<T: Iterator> Transform for T {}
