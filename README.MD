# topological_peak_detection

This crate finds peaks inside of 1-D signals (vectors of `PartialOrd` satisfying types) using topological persistance.

## What it does:
 - Returns peaks and regions in a 1-D signal.
 - Should work on all types which can be sorted.

## What it doesn't do:
 - it doesn't tolerance peaks, maybe it should maybe it shouldn't.
 - it doesn't apply to 2 or higher dimensions. It easily can, but, currently, it doesn't.
 - it does not merge homology's for birth/death analysis. It easily could.

Here's a simple example:
# Examples
```rust
use core::f32::consts::PI;
...
let tst_vec:Vec<f32> = (0..6001)
            .map(|x| ((x as f32 / 1000_f32) * PI).sin())
            .collect();
let homologies = find_homologies(&tst_vec);
let x = get_peaks(&homologies);
// x == [500, 2500, 4500, 6000]
```