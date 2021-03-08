use itertools::Itertools;

use crate::geometry::{Distance, Point, PointPair};

pub mod task_1;
pub mod task_3_1;
pub mod task_3_2;
pub mod task_4;

/// Finds the minimum distance in a 'strip' of distances.
///
/// This strip is defined as an iterator of
/// all the points within the x-distance of the current median, sorted by their y-coordinates.
/// Because of the nature of the divide-and-conquer algorithm, it is proven that this the loop
/// runs at most six times.
/// # Returns
/// Function returns `None` if there are no points better than the `current_distance` provided.
pub fn find_minimum_in_strip<'a>(
    points: impl Iterator<Item = &'a Point> + Clone,
    current_minimum: Distance,
) -> Option<PointPair> {
    let mut minimum_distance = current_minimum;
    let mut minimum_pair: Option<PointPair> = None;

    // Sub-iter is a copy of the iterators in the strip
    let mut sub_iter = points.clone().fuse();

    for (a, b) in points.flat_map(move |x| {
        // Do not take the item we're on (skip 1)
        sub_iter.next();
        // Repeat the current point, plus the next N items until out of bounds of strip
        std::iter::repeat(x).zip(
            sub_iter
                .clone()
                .take_while(move |&&p| (p.y - x.y) < minimum_distance.0),
        )
    }) {
        if a.distance_to(*b) < minimum_distance {
            minimum_distance = a.distance_to(*b);
            minimum_pair = Some(PointPair(*a, *b));
        }
    }
    minimum_pair
}

/// Performs a brute-force calculation to find the closest pair of points within the iterator.
///
/// This will run in `O(n^2)` time, so it is only used for the base case of the recursive algorithm.
/// # Panics
/// Function will panic if the iterator is empty.
pub fn find_minimum_bruteforce<'a>(points: impl Iterator<Item = &'a Point> + Clone) -> PointPair {
    points
        .combinations(2)
        .map(|x| PointPair(*x[0], *x[1]))
        .sorted_by(|p1, p2| p1.distance().partial_cmp(&p2.distance()).unwrap())
        .next()
        .unwrap()
}
