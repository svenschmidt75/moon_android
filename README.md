# Introduction

This project implements various algorithms from book "Astronomical Algorithms" by Jean Meeus, to show
Moon-related information in an Android app.
These calculations are implemented in Rust, whereas the Android app is using Kotlin.

### delta_t_converter

The Rust crate contains a package called *delta_t_converter*.
Periodically, the delta_t data to calculate TT from UTC is updated
by NASA. The file is [here](https://cddis.nasa.gov/archive/products/iers/finals2000A.all).
To convert, run

```
cargo run run --package delta_t_converter --bin delta_t_converter -- ../finals2000A.all
```
where file ```finals2000A.all``` is a ASCII copy of the data file.
The output file ```finals2000A.all.rs``` contains the delta_t structure elements that need
to be placed in ```tabular/src/time/delta_t_data.rs```.

### delta_t_pred_converter

Future delta t values which are not covered in [finals2000A.all](https://cddis.nasa.gov/archive/products/iers/finals2000A.all)
are given in [deltat.preds](https://cddis.nasa.gov/archive/products/iers/deltat.preds).
In this file, the delta t values are given directly, rather than delta ut = UT1 - UTC as is the case
with [finals2000A.all](https://cddis.nasa.gov/archive/products/iers/finals2000A.all).
To convert, run
s
```
cargo run run --package delta_t_pred_converter --bin delta_t_pred_converter -- ../deltat.preds
```
where file ```deltat.preds``` is a ASCII copy of the data file.
The output file ```deltat.preds.rs``` contains the delta_t structure elements that need
to be placed in ```tabular/src/time/delta_t_data.rs```, at the very bottom.

Note that there might be overlap between the ```DeltaTValue``` items from
[finals2000A.all](https://cddis.nasa.gov/archive/products/iers/finals2000A.all)
and [deltat.preds](https://cddis.nasa.gov/archive/products/iers/deltat.preds). Delete the
overlapping values from [deltat.preds](https://cddis.nasa.gov/archive/products/iers/deltat.preds)
before pasting into ```tabular/src/time/delta_t_data.rs```.

# Credits

The implementations of astronomical algorithms is based on the book *Astronomical Algorithms*, Jean Meeus,
2nd edition, Willmann-Bell Inc.
I heavily relied on the project [AA+](http://www.naughter.com/aa.html) by PJ Naughter.