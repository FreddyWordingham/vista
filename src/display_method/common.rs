//! ### Common formatting functionality
//!
//! This module contains shared formatting logic used by all display methods.

use ndarray::{ArrayBase, Data, Dimension};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
    DisplayArray,
    display_method::{DisplayMethod, formatter::ElementFormatter},
};

/// Format a single array using the given element formatter.
#[inline]
pub fn fmt_single_array<S, D, F>(arr: &ArrayBase<S, D>, f: &mut Formatter<'_>, element_formatter: &F) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
    F: ElementFormatter,
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
        let is_last_in_row = (i + 1) % row_size == 0;
        element_formatter.format_element(f, elem, width, is_last_in_row)?;

        // newline at end of any axis-block, but not after the very last element
        for &chunk in &chunk_sizes {
            if (i + 1) % chunk == 0 && (i + 1) < total {
                writeln!(f)?;
            }
        }
    }

    Ok(())
}

/// Format multiple arrays using the given element formatter.
#[inline]
pub fn fmt_multiple_arrays<S, D, F>(arrays: &[&ArrayBase<S, D>], f: &mut Formatter<'_>, element_formatter: &F) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
    F: ElementFormatter,
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
    for arr in arrays {
        let width = arr.iter().map(|e| e.to_string().len()).max().unwrap_or(0);
        array_widths.push(width);
    }

    // Special case for 1D arrays
    if ndim == 1 {
        for (arr_idx, arr) in arrays.iter().enumerate() {
            for (i, elem) in arr.iter().enumerate() {
                let is_last_in_row = i == arr.len() - 1;
                element_formatter.format_element(f, elem, array_widths[arr_idx], is_last_in_row)?;
            }

            if arr_idx < arrays.len() - 1 {
                element_formatter.write_array_separator(f)?;
            }
        }
        return Ok(());
    }

    // Handle multi-dimensional arrays by recursive traversal
    recursive_format_arrays(
        f,
        arrays,
        &array_widths,
        first_shape,
        0,
        &mut vec![0; ndim],
        element_formatter,
    )
}

/// Recursively format multi-dimensional arrays.
#[inline]
fn recursive_format_arrays<S, D, F>(
    f: &mut Formatter<'_>,
    arrays: &[&ArrayBase<S, D>],
    array_widths: &[usize],
    shape: &[usize],
    level: usize,
    indices: &mut Vec<usize>,
    element_formatter: &F,
) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
    F: ElementFormatter,
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
                    let is_last_in_row = col == shape[level + 1] - 1;
                    element_formatter.format_element(f, elem, array_widths[arr_idx], is_last_in_row)?;
                }

                // Space between arrays
                if arr_idx < arrays.len() - 1 {
                    element_formatter.write_array_separator(f)?;
                }
            }
            writeln!(f)?;
        }
        return Ok(());
    }

    // Process higher dimensions recursively
    for i in 0..shape[level] {
        indices[level] = i;
        recursive_format_arrays(f, arrays, array_widths, shape, level + 1, indices, element_formatter)?;

        // Add extra newline between blocks except after the last one
        if i < shape[level] - 1 {
            writeln!(f)?;
        }
    }

    Ok(())
}

/// Implementation of Display for `DisplayArray` with a generic formatter.
#[inline]
pub fn display_impl<S, D, F>(
    display_array: &DisplayArray<'_, S, D, F>,
    f: &mut Formatter<'_>,
    element_formatter: &F,
) -> FmtResult
where
    S: Data,
    S::Elem: Display,
    D: Dimension,
    F: DisplayMethod + ElementFormatter,
{
    if display_array.arrays.len() == 1 {
        return fmt_single_array(display_array.arrays[0], f, element_formatter);
    }
    fmt_multiple_arrays(&display_array.arrays, f, element_formatter)
}
