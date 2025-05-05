//! ### `CommaSeparated` Display Method
//!
//! This module contains the `CommaSeparated` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{DisplayArray, display_method::DisplayMethod};

/// Display method that prints each element with a comma and space as a separator.
#[non_exhaustive]
pub struct CommaSeparated;

impl DisplayMethod for CommaSeparated {}

impl<S, D> Display for DisplayArray<'_, S, D, CommaSeparated>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let arr = self.array;
        let shape = arr.shape();
        let ndim = shape.len();

        // 1) compute widths for padding
        let width = arr.iter().map(|e| e.to_string().len()).max().unwrap_or(0);

        // 2) chunk sizes: one per axis (except last) = prod(shape[k+1..])
        let chunk_sizes: Vec<usize> = (0..ndim.saturating_sub(1)).map(|k| shape[k + 1..].iter().product()).collect();

        // for 1D use full length, else first chunk
        let row_size = if ndim == 1 { shape[0] } else { chunk_sizes[0] };

        // total elements to suppress final newline
        let total = arr.len();

        for (i, elem) in arr.iter().enumerate() {
            let s = elem.to_string();
            write!(f, "{s:>width$}")?;

            // comma-space unless end of row
            if (i + 1) % row_size != 0 {
                write!(f, ", ")?;
            }

            // newline at end of any axis‐block, but not after the very last element
            for &chunk in &chunk_sizes {
                if (i + 1) % chunk == 0 && (i + 1) < total {
                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}
