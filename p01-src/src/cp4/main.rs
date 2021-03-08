use closest_pairs::bail_error;
use closest_pairs::parse::parse_stdin;
use std::io;
use std::io::Read;
use closest_pairs::closest::task_4::Task4;

fn main() {
    let mut buffer = String::new();

    // Read from stdin
    io::stdin()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|_| bail_error!(1));

    // Parse and create struct
    let points = {
        let (_, points) = parse_stdin(&buffer).unwrap_or_else(|_| bail_error!(1));
        Task4::new(points)
    };

    // Calculate
    let closest = points.find_closest_pair();

    unsafe {
        closest.distance().pretty_print();
    }
}
