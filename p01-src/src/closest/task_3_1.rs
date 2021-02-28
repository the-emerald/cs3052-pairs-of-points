use crate::closest::{find_minimum_bruteforce, find_minimum_in_strip};
use crate::geometry::{Point, PointPair};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Task3QuickSort {
    points: Vec<Point>,
}

impl Task3QuickSort {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    // Entry function
    pub fn find_closest_pair(&mut self) -> PointPair {
        self.points
            .sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        find_closest_pair_inner(&mut self.points)
    }
}

fn find_closest_pair_inner(points: &mut [Point]) -> PointPair {
    // Reference: W2 L3

    // Base case: we can't recurse any further
    if points.len() <= 3 {
        return find_minimum_bruteforce(points.iter());
    }

    // We have already sorted the entire array
    let length = points.len();
    let (left, right) = points.split_at_mut(length / 2);
    let median = *right.first().unwrap();

    // Recursively solve the problem by left and right
    let left_minimum = find_closest_pair_inner(left);

    let right_minimum = find_closest_pair_inner(right);

    // Get minimum distance
    let minimum = left_minimum.min(right_minimum);

    // Filter out all points not in the "strip", sort by y coordinate.
    let strip = points
        .iter()
        .filter(|p| (p.x - median.x).abs() < minimum.distance().0)
        .sorted_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let minimum_in_strip = find_minimum_in_strip(strip, minimum.distance());

    match minimum_in_strip {
        Some(m) => m,
        None => minimum,
    }
}
