//! ## `DisplayExt`
//!
//! This module provides the `DisplayExt` trait, which is used to add a `display` convenience method to `ndarray` arrays.

use ndarray::{ArrayBase, RawData};

use crate::{DisplayArray, display_method::DisplayMethod};

/// Common trait for displayable arrays.
pub trait DisplayExt<T: RawData, D> {
    /// Construct a `DisplayArray` type for the given data.
    fn display<M: DisplayMethod>(&self) -> DisplayArray<'_, T, D, M>;
}

impl<T: RawData, D> DisplayExt<T, D> for ArrayBase<T, D> {
    #[inline]
    fn display<M: DisplayMethod>(&self) -> DisplayArray<'_, T, D, M> {
        DisplayArray::new(vec![&self])
    }
}

impl<T: RawData, D, const N: usize> DisplayExt<T, D> for [&ArrayBase<T, D>; N] {
    #[inline]
    fn display<M: DisplayMethod>(&self) -> DisplayArray<'_, T, D, M> {
        DisplayArray::new(self.to_vec())
    }
}

impl<T: RawData, D> DisplayExt<T, D> for &[&ArrayBase<T, D>] {
    #[inline]
    fn display<M: DisplayMethod>(&self) -> DisplayArray<'_, T, D, M> {
        DisplayArray::new(self.to_vec())
    }
}
