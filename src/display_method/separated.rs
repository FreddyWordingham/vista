//! ### `Separated` Display Method
//!
//! This module contains the `Separated` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    DisplayArray,
    display_method::{DisplayMethod, common, formatter::ElementFormatter},
};

/// Display method that prints each element separated by a space.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Separated;

impl DisplayMethod for Separated {}

impl ElementFormatter for Separated {
    fn format_element<T: Display>(&self, f: &mut Formatter<'_>, elem: &T, width: usize, is_last_in_row: bool) -> FmtResult {
        let s = elem.to_string();
        write!(f, "{s:>width$}")?;

        // space unless end of row
        if !is_last_in_row {
            write!(f, " ")?;
        }

        Ok(())
    }
}

impl<S, D> Display for DisplayArray<'_, S, D, Separated>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        common::display_impl(self, f, &Separated)
    }
}
