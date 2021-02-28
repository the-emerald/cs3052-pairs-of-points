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
    let (left, right) = points.split_at_mut(length / 2);
    let median = *right.first().unwrap();

    // Recursively solve the problem by left and right
    let left_minimum = find_closest_pair_inner(left);

    let right_minimum = find_closest_pair_inner(right);

    // Get minimum distance
    let minimum = left_minimum.min(right_minimum);

    // Sort by y first
    // points.sort_unstable_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

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
