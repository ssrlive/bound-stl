# Bound-STL

Bound-STL attempts to implement `lower_bound` and `upper_bound` in C++ STL.

This repo hosts at [bound-stl](https://github.com/ssrlive/bound-stl/)

[![Version](https://img.shields.io/crates/v/bound-stl.svg?style=flat)](https://crates.io/crates/bound-stl)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/bound-stl)
[![License](https://img.shields.io/crates/l/bound-stl.svg?style=flat)](https://github.com/ssrlive/bound-stl/blob/master/LICENSE)

## Usage

```rust
use bound_stl::LowerBound;

let v = vec![1, 2, 3, 4, 5];
assert_eq!(v.lower_bound(&3), Some(2));
assert_eq!(v.lower_bound(&6), Err(5));


use bound_stl::UpperBound;

let v = vec![1, 2, 3, 4, 5];
assert_eq!(v.upper_bound(&3), Some(3));
assert_eq!(v.upper_bound(&6), Err(5));

```
