use crate::closest::{find_minimum_bruteforce, find_minimum_in_strip};
use crate::geometry::{Point, PointPair};

#[derive(Clone, Debug)]
pub struct Task3SortedY {
    points: Vec<Point>,
}

impl Task3SortedY {
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
        points.sort_unstable_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
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
    points.copy_from_slice(&points_cpy);

    // Check invariant!
    debug_assert!((0..points.len() - 1).all(|i| points[i].y <= points[i + 1].y));

    // Now filter
    let strip = points
        .iter()
        .filter(|p| (p.x - median.x).abs() < minimum.distance().0);

    let minimum_in_strip = find_minimum_in_strip(strip, minimum.distance());

    match minimum_in_strip {
        Some(m) => m,
        None => minimum,
    }
}

pub fn merge(left: &[Point], right: &[Point], points: &mut [Point]) {
    debug_assert_eq!(left.len() + right.len(), points.len());

    let (mut i, mut j, mut k) = (0, 0, 0);

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

    if i < left.len() {
        points[k..].copy_from_slice(&left[i..]);
    }
    if j < right.len() {
        points[k..].copy_from_slice(&right[j..]);
    }
}
