use std::io;
use std::io::Read;
use closest_pairs::parse::parse_stdin;
use closest_pairs::bail_error;
use closest_pairs::closest::closest_v1::ClosestV1;

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("could not read stdin");

    let mut points = {
        let (_, points) = parse_stdin(&buffer)
            .unwrap_or_else(|e| bail_error!(e, 1));
        ClosestV1::new(points)
    };

    let closest = points.find_closest_pair();

    dbg!(closest);
}
