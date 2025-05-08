//! ### `Joined` Display Method
//!
//! This module contains the `Joined` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    DisplayArray,
    display_method::{DisplayMethod, common, formatter::ElementFormatter},
};

/// Display method that prints each element without any separator.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Joined;

impl DisplayMethod for Joined {}

impl ElementFormatter for Joined {
    fn format_element<T: Display>(&self, f: &mut Formatter<'_>, elem: &T, _width: usize, _is_last_in_row: bool) -> FmtResult {
        write!(f, "{elem}")
    }
}

impl<S, D> Display for DisplayArray<'_, S, D, Joined>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        common::display_impl(self, f, &Joined)
    }
}
