# Vista

A utility library for displaying and formatting arrays, matrices and tensors in the terminal.

[![Crates.io](https://img.shields.io/crates/v/vista.svg)](https://crates.io/crates/vista)
[![Documentation](https://docs.rs/vista/badge.svg)](https://docs.rs/vista)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Overview

Vista provides flexible, readable formatting options for ndarray arrays in Rust. Whether you're working with 1D vectors, 2D matrices, or higher-dimensional tensors, Vista makes it easy to display your array data clearly in the terminal.

## Features

- 📊 Support for arrays of all dimensions (1D, 2D, 3D, 4D, and higher)
- 🎨 Multiple display formats to choose from
- 🧰 Simple, ergonomic API
- ⚡ Zero dependencies beyond ndarray itself
- 📝 Comprehensive documentation and examples

## Installation

Add Vista to your `Cargo.toml`:

```toml
[dependencies]
vista = "0.1.0"
ndarray = "0.16.1"
```

## Quick Start

```rust
use ndarray::arr2;
use vista::{DisplayExt, Separated};

fn main() {
    // Create a 2x3 matrix
    let matrix = arr2(&[[1.0, 2.0, 3.0],
                         [4.0, 5.0, 6.0]]);

    // Display it with space separation
    println!("{}", matrix.display::<Separated>());
}
```

## Display Methods

Vista provides four display methods out of the box:

### `Separated`

Displays elements with space separation and aligned columns:

```
1.0 2.0 3.0
4.0 5.0 6.0
```

### `CommaSeparated`

Displays elements with comma separation and aligned columns:

```
1.0, 2.0, 3.0
4.0, 5.0, 6.0
```

### `Joined`

Displays elements without separators:

```
1.02.03.0
4.05.06.0
```

### `DoubleJoined`

Displays each element twice with no separators:

```
1.01.02.02.03.03.0
4.04.05.05.06.06.0
```

> Note: This is useful for displaying image data.

## Examples

### 1D Array (Vector)

```rust
use ndarray::arr1;
use vista::{DisplayExt, Separated};

let vector = arr1(&[1.0, 2.0, 3.0, 4.0]);
println!("{}", vector.display::<Separated>());
// Output: 1.0 2.0 3.0 4.0
```

### 2D Array (Matrix)

```rust
use ndarray::arr2;
use vista::{DisplayExt, CommaSeparated};

let matrix = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
println!("{}", matrix.display::<CommaSeparated>());
// Output:
// 1.0, 2.0
// 3.0, 4.0
```

### 3D Array (Tensor)

```rust
use ndarray::arr3;
use vista::{DisplayExt, Separated};

let tensor = arr3(&[[[1.0, 2.0], [3.0, 4.0]],
                    [[5.0, 6.0], [7.0, 8.0]]]);
println!("{}", tensor.display::<Separated>());
// Output:
// 1.0 2.0
// 3.0 4.0
//
// 5.0 6.0
// 7.0 8.0
```
