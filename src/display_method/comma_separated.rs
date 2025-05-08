//! ### `CommaSeparated` Display Method
//!
//! This module contains the `CommaSeparated` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::{DisplayMethod, common, formatter::ElementFormatter};
use crate::DisplayArray;

/// Display method that prints each element with a comma and space as a separator.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct CommaSeparated;

impl DisplayMethod for CommaSeparated {}

impl ElementFormatter for CommaSeparated {
    fn format_element<T: Display>(&self, f: &mut Formatter<'_>, elem: &T, width: usize, is_last_in_row: bool) -> FmtResult {
        let s = elem.to_string();
        write!(f, "{s:>width$}")?;

        // comma-space unless end of row
        if !is_last_in_row {
            write!(f, ", ")?;
        }

        Ok(())
    }
}

impl<S, D> Display for DisplayArray<'_, S, D, CommaSeparated>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        common::display_impl(self, f, &CommaSeparated)
    }
}
