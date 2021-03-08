use crate::closest::{find_minimum_bruteforce, find_minimum_in_strip};
use crate::geometry::{Point, PointPair};

/// Task 3 (1): Divide and conquer, pre-sort points by x-coordinates, then sort by y-coordinates
/// on the way up the call tree.
#[derive(Clone, Debug)]
pub struct Task3SortedY {
    points: Vec<Point>,
}

impl Task3SortedY {
    /// Create a new `Task3(2)` struct.
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Find the closest pair of points in the list.
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
        points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        return find_minimum_bruteforce(points.iter());
    }

    // We have already sorted the entire array
    let length = points.len();
    let (left_x, right_x) = points.split_at_mut(length / 2);
    let median = *right_x.first().unwrap();

    let left_x_len = left_x.len();

    // Recursively solve the problem by left and right
    let left_minimum = find_closest_pair_inner(left_x);

    let right_minimum = find_closest_pair_inner(right_x);

    // Get minimum distance
    let minimum = left_minimum.min(right_minimum);

    // Merge
    let mut points_cpy = points.to_vec();
    merge(
        &points[0..left_x_len],
        &points[left_x_len..],
        &mut points_cpy,
    );
    points.copy_from_slice(&points_cpy);    // memcpy

    // Check invariant. All points should be sorted by y-coordinates
    debug_assert!((0..points.len() - 1).all(|i| points[i].y <= points[i + 1].y));

    // Filter to strip
    let strip = points
        .iter()
        .filter(|p| (p.x - median.x).abs() < minimum.distance().0);

    match find_minimum_in_strip(strip, minimum.distance()) {
        Some(m) => m,
        None => minimum,
    }
}

/// Merges two sorted slices, `left` and `right`, into the slice `points`.
/// # Panics
/// Function will panic if the length of the left and right slices do not sum to the final slice.
pub fn merge(left: &[Point], right: &[Point], points: &mut [Point]) {
    debug_assert_eq!(left.len() + right.len(), points.len());

    let (mut i, mut j, mut k) = (0, 0, 0);

    // Do merge
    while i < left.len() && j < right.len() {
        if left[i].y <= right[j].y {
            points[k] = left[i];
            k += 1;
            i += 1;
        } else {
            points[k] = right[j];
            k += 1;
            j += 1;
        }
    }

    // Copy leftover items
    if i < left.len() {
        points[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        points[k..].copy_from_slice(&right[j..]);
    }
}
