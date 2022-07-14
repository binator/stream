#![doc = include_str!("../readme.md")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::ops::{
  Range,
  RangeFrom,
};

mod reader_stream;
pub use reader_stream::*;

mod vec_stream;
pub use vec_stream::*;
// mod string_stream;
// pub use string_stream::*;

// #[cfg(feature = "bit_stream")]
// mod bit_stream;
// #[cfg(feature = "bit_stream")]
// pub use bit_stream::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Position {
  RangeFrom(RangeFrom<usize>),
  Range(Range<usize>),
}

impl Position {
  fn next(&self) -> (Option<usize>, Self) {
    match self {
      Position::RangeFrom(from) => {
        let mut from = from.clone();
        (from.next(), Position::RangeFrom(from))
      }
      Position::Range(range) => {
        let mut range = range.clone();
        (range.next(), Position::Range(range))
      }
    }
  }

  fn range(&self, b: &Self) -> Option<Range<usize>> {
    let (a, b) = match (self, b) {
      (Position::RangeFrom(a), Position::RangeFrom(b)) => (a.start, b.start),
      (Position::RangeFrom(a), Position::Range(b)) => (a.start, b.start),
      (Position::Range(a), Position::RangeFrom(b)) => (a.start, b.start),
      (Position::Range(a), Position::Range(b)) => (a.start, b.start),
    };

    if a >= b { Some(a..a - b) } else { None }
  }
}
