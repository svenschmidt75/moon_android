# Introduction

This project implements various algorithms from book "Astronomical Algorithms" by Jean Meeus, to show
Moon-related information in an Android app.
These calculations are implemented in Rust, whereas the Android app is using Kotlin.

# delta_t_converter

The Rust crate contains a package called *delta_t_converter*.
Periodically, the delta_t data to calculate TT from UTC is updated
by NASA. The file is [here](https://cddis.nasa.gov/archive/products/iers/finals2000A.all).
To convert, run

```
cargo run run --package delta_t_converter --bin delta_t_converter -- ../finals2000A.all
```
where file ```finals2000A.all``` is a ASCII copy of the data file.
The output file ```finals2000A.rs``` contains the delta_t structure elements that need
to be placed in ```tabular/src/time/delta_t_data.rs```.
