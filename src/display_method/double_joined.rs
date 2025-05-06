//! ### `DoubleJoined` Display Method
//!
//! This module contains the `DoubleJoined` display method for formatting arrays.

use ndarray::{ArrayBase, Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{DisplayArray, display_method::DisplayMethod};

/// Display method that prints each element twice, without any separator.
#[non_exhaustive]
pub struct DoubleJoined;

impl DisplayMethod for DoubleJoined {}

impl<S, D> Display for DisplayArray<'_, S, D, DoubleJoined>
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.arrays.len() == 1 {
            return fmt_single_array(self.arrays[0], f);
        }
        fmt_multiple_arrays(&self.arrays, f)
    }
}

/// Format a single array with double joined elements.
#[inline]
fn fmt_single_array<S, D>(arr: &ArrayBase<S, D>, f: &mut Formatter<'_>) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    let shape = arr.shape();
    let ndim = shape.len();

    // chunk sizes for all but the last axis
    let chunk_sizes: Vec<usize> = (0..ndim.saturating_sub(1)).map(|k| shape[k + 1..].iter().product()).collect();

    // total elements, to avoid a trailing newline
    let total = arr.len();

    for (i, elem) in arr.iter().enumerate() {
        let s = elem.to_string();
        // print each element doubled
        write!(f, "{s}{s}")?;

        // newline at the end of any axis-block, but not after the very last element
        for &chunk in &chunk_sizes {
            if (i + 1) % chunk == 0 && (i + 1) < total {
                writeln!(f)?;
            }
        }
    }

    Ok(())
}

/// Format multiple arrays inline with double joined elements.
#[inline]
fn fmt_multiple_arrays<S, D>(arrays: &[&ArrayBase<S, D>], f: &mut Formatter<'_>) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
    if arrays.is_empty() {
        return Ok(());
    }

    // Make sure all arrays have the same shape
    let first_shape = arrays[0].shape();
    for (i, arr) in arrays.iter().enumerate().skip(1) {
        if arr.shape() != first_shape {
            return write!(
                f,
                "Error: Arrays have different shapes. Array 0 shape: {:?}, Array {} shape: {:?}",
                first_shape,
                i,
                arr.shape()
            );
        }
    }

    let ndim = first_shape.len();

    // Special case for 1D arrays
    if ndim == 1 {
        for (arr_idx, arr) in arrays.iter().enumerate() {
            for elem in arr.iter() {
                let s = elem.to_string();
                // Print each element doubled
                write!(f, "{s}{s}")?;
            }

            if arr_idx < arrays.len() - 1 {
                write!(f, "  ")?;
            }
        }
        return Ok(());
    }

    // Handle multi-dimensional arrays by recursive traversal
    fn write_array_level<S, D>(
        f: &mut Formatter<'_>,
        arrays: &[&ArrayBase<S, D>],
        shape: &[usize],
        level: usize,
        indices: &mut Vec<usize>,
    ) -> FmtResult
    where
        S: Data,
        S::Elem: Display,
        D: Dimension,
    {
        if level == shape.len() - 2 {
            // At the row level (second-to-last dimension)
            for row in 0..shape[level] {
                indices[level] = row;

                // Process each array at this row
                for (arr_idx, arr) in arrays.iter().enumerate() {
                    // Print each column in this row
                    for col in 0..shape[level + 1] {
                        indices[level + 1] = col;

                        // Calculate flattened index
                        let mut flat_index = 0;
                        let mut multiplier = 1;
                        for d in (0..shape.len()).rev() {
                            flat_index += indices[d] * multiplier;
                            multiplier *= shape[d];
                        }

                        let elem = arr.iter().nth(flat_index).unwrap();
                        let s = elem.to_string();
                        // Print each element doubled
                        write!(f, "{s}{s}")?;
                    }

                    // Space between arrays
                    if arr_idx < arrays.len() - 1 {
                        write!(f, "  ")?;
                    }
                }
                writeln!(f)?;
            }
            return Ok(());
        }

        // Process higher dimensions recursively
        for i in 0..shape[level] {
            indices[level] = i;
            write_array_level(f, arrays, shape, level + 1, indices)?;

            // Add extra newline between blocks except after the last one
            if i < shape[level] - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }

    // Initialize indices vector for tracking position
    let mut indices = vec![0; ndim];

    // Start recursive traversal at the outer dimension
    write_array_level(f, arrays, first_shape, 0, &mut indices)
}
