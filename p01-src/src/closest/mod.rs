use itertools::Itertools;

use crate::geometry::{Distance, Point, PointPair};

pub mod task_1;
pub mod task_3_1;
pub mod task_3_2;

pub(crate) fn find_minimum_in_strip<'a>(
    points: impl Iterator<Item = &'a Point> + Clone,
    current_minimum: Distance,
) -> Option<PointPair> {
    let mut minimum_distance = current_minimum;
    let mut minimum_pair: Option<PointPair> = None;

    let mut sub_iter = points.clone().fuse();
    for (a, b) in points.flat_map(move |x| {
        sub_iter.next();
        std::iter::repeat(x).zip(sub_iter.clone())
    }) {
        if a.distance_to(*b) < minimum_distance {
            minimum_distance = a.distance_to(*b);
            minimum_pair = Some(PointPair(*a, *b));
        }
    }
    minimum_pair
}

pub(crate) fn find_minimum_bruteforce<'a>(
    points: impl Iterator<Item = &'a Point> + Clone,
) -> PointPair {
    points
        .combinations(2)
        .map(|x| PointPair(*x[0], *x[1]))
        .sorted_by(|p1, p2| p1.distance().partial_cmp(&p2.distance()).unwrap())
        .next()
        .unwrap()
}
