use closest_pairs::bail_error;
use closest_pairs::closest::closest_v1::ClosestV1;
use closest_pairs::parse::parse_stdin;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("could not read stdin");

    let mut points = {
        let (_, points) = parse_stdin(&buffer).unwrap_or_else(|e| bail_error!(e, 1));
        ClosestV1::new(points)
    };

    // dbg!(&points);

    let closest = points.find_closest_pair();

    // dbg!(closest);
    // dbg!(closest.distance());

    println!("{}", closest.distance());
}
