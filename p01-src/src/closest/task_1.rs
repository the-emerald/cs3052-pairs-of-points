use itertools::Itertools;

use crate::closest;
use crate::geometry::{Point, PointPair};
use crate::quick_select::quick_select_points;

/// Task 1: Divide and conquer using QuickSelect
#[derive(Clone, Debug)]
pub struct Task1 {
    points: Vec<Point>,
}

impl Task1 {
    /// Create a new `Task1` struct.
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Find the closest pair of points in the list.
    pub fn find_closest_pair(&mut self) -> PointPair {
        find_closest_pair_inner(&mut self.points)
    }
}

fn find_closest_pair_inner(points: &mut [Point]) -> PointPair {
    // Reference: W2 L3

    // Base case: we can't recurse any further
    if points.len() <= 3 {
        return closest::find_minimum_bruteforce(points.iter());
    }

    // Use quickselect to find median point
    // Split points into two sets, lesser and greater
    let length = points.len();
    let (left, right) = quick_select_points(points, length / 2);
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

    // Return the new minimum if the strip had a better value
    match closest::find_minimum_in_strip(strip, minimum.distance()) {
        Some(m) => m,
        None => minimum,
    }
}
