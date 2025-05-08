//! ### `Formatter` trait
//!
//! This module contains the `Formatter` trait that defines the formatting behavior for different display methods.

use std::fmt::{Display, Formatter, Result};

/// Defines how elements are formatted when displayed
pub trait ElementFormatter {
    /// Format an element and write it to the formatter
    fn format_element<T: Display>(&self, f: &mut Formatter<'_>, elem: &T, width: usize, is_last_in_row: bool) -> Result;

    /// Write spacing between arrays when displaying multiple arrays
    fn write_array_separator(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "  ")
    }
}
