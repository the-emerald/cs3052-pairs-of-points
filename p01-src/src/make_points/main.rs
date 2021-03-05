use closest_pairs::geometry::Point;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

const SIZE: usize = 5_000_000;
const SEED: u64 = 0x456D_6D79;
const FILE_LOCATION: &str = "../stacs/testing.in";

fn main() {
    let mut rng = StdRng::seed_from_u64(SEED);
    let file = File::create(FILE_LOCATION).unwrap();
    let mut bw = BufWriter::new(file);

    writeln!(&mut bw, "{}", SIZE).unwrap();

    (0..SIZE)
        .map(|_| Point {
            x: rng.gen::<i64>() as f64,
            y: rng.gen::<i64>() as f64,
        })
        .for_each(|p| {
            writeln!(&mut bw, "{:.1} {:.1}", p.x, p.y).unwrap();
        })
}
