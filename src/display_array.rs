//! ## `DisplayArray`
//!
//! This module contains the `DisplayArray` struct, a wrapper type used to format array data in various ways.

use ndarray::{ArrayBase, RawData};
use std::marker::PhantomData;

use crate::display_method::DisplayMethod;

/// Display wrapper with a phantom type for the display method
#[non_exhaustive]
pub struct DisplayArray<'a, T: RawData, D, M: DisplayMethod> {
    /// The array to be displayed.
    pub arrays: Vec<&'a ArrayBase<T, D>>,
    /// Marker to hold the display method type.
    pub _phantom: PhantomData<M>,
}

impl<'a, T: RawData, D, M: DisplayMethod> DisplayArray<'a, T, D, M> {
    /// Construct a new `DisplayArray` instance referring to the given array.
    #[inline]
    pub const fn new(arrays: Vec<&'a ArrayBase<T, D>>) -> Self {
        DisplayArray {
            arrays,
            _phantom: PhantomData,
        }
    }
}
