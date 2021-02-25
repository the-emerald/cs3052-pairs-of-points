use crate::geometry::{Distance, Point, PointPair};
use crate::quick_select::quick_select_points;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct ClosestV1 {
    points: Vec<Point>,
}

impl ClosestV1 {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    // Entry function
    pub fn find_closest_pair(&mut self) -> PointPair {
        find_closest_pair_inner(&mut self.points)
    }
}

fn find_closest_pair_inner(points: &mut [Point]) -> PointPair {
    // Reference: W2 L3
    dbg!(points.len());
    dbg!(&points);

    // Base case: we can't recurse any further
    if points.len() <= 3 {
        dbg!("brute force required");
        return find_minimum_bruteforce(points.iter());
    }

    // Use quickselect to find median point
    // Split points into two sets, lesser and greater
    let length = points.len();
    let (left, right) = quick_select_points(points, length / 2);
    let pivot = {
        let l = left.len() - 1;
        left[l]
    };

    // Recursively solve the problem by left and right
    let left_minimum = find_closest_pair_inner(left);

    let right_minimum = find_closest_pair_inner(right);

    // Get minimum distance
    let minimum = left_minimum.min(right_minimum);

    // Filter out all points not in the "strip", sort by y coordinate.
    let strip = left
        .iter()
        .chain(right.iter())
        .filter(|p| (p.x - pivot.x).abs() < minimum.distance().0)
        .sorted_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    dbg!(strip.clone().collect_vec().len());

    let minimum_in_strip = find_minimum_in_strip(strip);

    (minimum_in_strip).min(minimum)
}

fn find_minimum_in_strip<'a>(points: impl Iterator<Item = &'a Point> + Clone) -> PointPair {
    let mut minimum_distance = Distance(f64::MAX);
    let mut minimum_pair = (None, None);

    let mut sub_iter = points.clone().fuse();
    for (a, b) in points.flat_map(move |x| {
        sub_iter.next();
        std::iter::repeat(x).zip(sub_iter.clone())
    }) {
        if a.distance_to(*b) < minimum_distance {
            minimum_distance = a.distance_to(*b);
            minimum_pair = (Some(a), Some(b));
        }
    }
    dbg!(minimum_distance);

    PointPair(*minimum_pair.0.unwrap(), *minimum_pair.1.unwrap())
}

fn find_minimum_bruteforce<'a>(points: impl Iterator<Item = &'a Point> + Clone) -> PointPair {
    points
        .combinations(2)
        .map(|x| PointPair(*x[0], *x[1]))
        .sorted_by(|p1, p2| p2.distance().partial_cmp(&p1.distance()).unwrap())
        .nth(0)
        .unwrap()
}