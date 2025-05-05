//! ### `Joined` Display Method
//!
//! This module contains the `Joined` display method for formatting arrays.

use ndarray::{Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{DisplayArray, display_method::DisplayMethod};

/// Display method that prints each element without any separator.
#[non_exhaustive]
pub struct Joined;

impl DisplayMethod for Joined {}

impl<S, D> Display for DisplayArray<'_, S, D, Joined>
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

        // chunk sizes for all but the last axis
        let chunk_sizes: Vec<usize> = (0..ndim.saturating_sub(1)).map(|k| shape[k + 1..].iter().product()).collect();

        // total elements, to avoid a trailing newline
        let total = arr.len();

        for (i, elem) in arr.iter().enumerate() {
            write!(f, "{elem}")?;

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
