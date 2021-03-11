# CS3052 - Practical 1

## Installing Rust
https://rustup.rs/ will provide instructions depending on your OS.

## Building and running
**For all cargo commands, prepend with `nolimit` as necessary on school host servers!**

Before running stacscheck, it is recommended that you build this project once as
the compiler needs to download and compile all dependencies first (and that may time
out the test).

`cargo build && cargo build --release`

To run any of the executables as defined by the spec:

`cargo run --release --bin=[cp1/cp3/cp4]`

## Collecting data
`criterion` (mainly), and a host of other tools are used to gather the results on
the report. To recreate these for yourself:

### Graphs and HTML report
These rely on `criterion`. To generate a set of data for yourself, first ensure that
you are on a quiet machine with no other major tasks running. Install:

`cargo install cargo-criterion`

and then run `cargo criterion` to run the battery of tests. The report will be generated
at `p01-src/target/criterion/reports`.

*Note: these tests may take longer than expected to run on a slow machine! Adjust 
values in `p01-src/benches/benchmarks.rs` under the `criterion_group!` macro, `config`.*

### Flamegraph
To generate a flamegraph, you will need `perf` installed on Linux alongside the
appropriate permissions. Install:

`cargo install flamegraph`

and then run `cargo flamegraph -o [output_filename.svg] --bin=[cp1/cp3/cp4]`.

### One-off wall-time measurement
`hyperfine` is used to generate these results. To install:

`cargo install hyperfine`

and then run `hyperfine --warmup-runs N 'target/release/[cp1/cp3/cp4] < path/to/input/file''`.
Repeat the final command as necessary to run multiple executables at the same time.

## Multithreaded Task 4
The report mentions a multithread-capable improvement on Task 4. You can find it on the
`parallel` branch of this repository.

`git checkout parallel`
`git checkout master`

`cp4` will spawn as many threads as the number of CPUs on the system.

## Generating large data sets
`make_points` is a binary that generates random points using a PRNG. **Before using,
create a directory called `stacs` in the root directory.**
  
Test vector: Using seed `0xABAB_BABA` and `10_000_000` points should create a file
with SHA1 hash `28defa510a7d9ea12e940e45d6011570e39c0e9b`. The first 5 points are:
```
10000000
6626793333355418624.0 -7496458538495980544.0
-8007577507249937408.0 278987807760401.0
8527392132699782144.0 -7145235817532835840.0
-5232829380667158528.0 -556649247614643072.0
-4316635244079889408.0 -1808188043019518208.0
```
