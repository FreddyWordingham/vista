//! ### `CommaSeparated` Display Method
//!
//! This module contains the `CommaSeparated` display method for formatting arrays.

use ndarray::{ArrayBase, Data, Dimension};
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
        if self.arrays.len() == 1 {
            return fmt_single_array(self.arrays[0], f);
        }
        fmt_multiple_arrays(&self.arrays, f)
    }
}

/// Format a single array with comma separation.
#[inline]
fn fmt_single_array<S, D>(arr: &ArrayBase<S, D>, f: &mut Formatter<'_>) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
{
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

/// Format multiple arrays inline with comma separation.
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

    // Calculate column widths for each array
    let mut array_widths: Vec<usize> = Vec::with_capacity(arrays.len());
    for arr in arrays.iter() {
        let width = arr.iter().map(|e| e.to_string().len()).max().unwrap_or(0);
        array_widths.push(width);
    }

    // Special case for 1D arrays
    if ndim == 1 {
        for (arr_idx, arr) in arrays.iter().enumerate() {
            for (i, elem) in arr.iter().enumerate() {
                let s = elem.to_string();
                write!(f, "{s:>width$}", width = array_widths[arr_idx])?;

                if i < arr.len() - 1 {
                    write!(f, ", ")?;
                }
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
        array_widths: &[usize],
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
                        write!(f, "{s:>width$}", width = array_widths[arr_idx])?;

                        // Comma unless last element in row
                        if col < shape[level + 1] - 1 {
                            write!(f, ", ")?;
                        }
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
            write_array_level(f, arrays, array_widths, shape, level + 1, indices)?;

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
    write_array_level(f, arrays, &array_widths, first_shape, 0, &mut indices)
}
