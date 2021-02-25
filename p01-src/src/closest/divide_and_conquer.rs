use crate::geometry::{Distance, Point};
use crate::quick_select::quick_select_points;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct DivideAndConquer {
    points: Vec<Point>,
}

impl DivideAndConquer {
    // Entry function
    pub fn find_closest_pair(&mut self) -> (&Point, &Point) {
        find_closest_pair_inner(&mut self.points)
    }
}

fn find_closest_pair_inner(points: &mut [Point]) -> (&Point, &Point) {
    // Reference: W2 L3

    // Use quickselect to find median point
    // Split points into two sets, lesser and greater
    let length = points.len();

    let (left, pivot, right) = quick_select_points(points, length / 2);

    // TODO: Determine if I'm allowed to use this.
    // let (left, pivot, right) = points.select_nth_unstable_by(length / 2, |fst, snd| {
    //     fst.x
    //         .partial_cmp(&snd.x)
    //         .expect("incredibly cursed f64 found")
    // });

    // Recursively solve the problem by left and right
    let left_minimum = {
        let (a, b) = find_closest_pair_inner(left);
        a.distance_to(*b) // TODO: Not so repetitive, please
    };

    let right_minimum = {
        let (a, b) = find_closest_pair_inner(right);
        a.distance_to(*b)
    };

    // Get minimum distance
    let minimum = Distance(left_minimum.0.min(right_minimum.0));

    // Filter out all points not in the "strip", sort by y coordinate.
    let left = left
        .iter()
        .filter(|p| (pivot.x - minimum.0) <= p.x && p.x <= pivot.x)
        .sorted_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let right = right
        .iter()
        .filter(|p| pivot.x <= p.x && p.x <= (pivot.x + minimum.0))
        .sorted_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    // Merge into the strip
    let strip = left.chain(right);

    let minimum_in_strip = find_minimum_in_strip(strip);

    todo!()
}

fn find_minimum_in_strip<'a>(points: impl Iterator<Item = &'a Point>) -> (&'a Point, &'a Point) {
    todo!()
}
