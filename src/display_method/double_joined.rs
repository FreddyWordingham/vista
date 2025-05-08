//! ### `DoubleJoined` Display Method
//!
//! This module contains the `DoubleJoined` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    DisplayArray,
    display_method::{DisplayMethod, common, formatter::ElementFormatter},
};

/// Display method that prints each element twice, without any separator.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct DoubleJoined;

impl DisplayMethod for DoubleJoined {}

impl ElementFormatter for DoubleJoined {
    fn format_element<T: Display>(&self, f: &mut Formatter<'_>, elem: &T, _width: usize, _is_last_in_row: bool) -> FmtResult {
        let s = elem.to_string();
        // print each element doubled
        write!(f, "{s}{s}")
    }
}

impl<S, D> Display for DisplayArray<'_, S, D, DoubleJoined>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        common::display_impl(self, f, &DoubleJoined)
    }
}
