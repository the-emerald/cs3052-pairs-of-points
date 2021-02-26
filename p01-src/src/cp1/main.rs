use closest_pairs::bail_error;
use closest_pairs::closest::task_1::Task1;
use closest_pairs::parse::parse_stdin;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|_| bail_error!(1));

    let mut points = {
        let (_, points) = parse_stdin(&buffer).unwrap_or_else(|_| bail_error!(1));
        Task1::new(points)
    };

    // dbg!(&points);

    let closest = points.find_closest_pair();

    // dbg!(closest);
    // dbg!(closest.distance());

    unsafe {
        closest.distance().pretty_print();
    }
}
