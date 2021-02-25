use crate::geometry::Point;
use itertools::partition;
use std::cmp::Ordering;

fn find_pivot(points: &[Point]) -> &Point {
    let f = &points[0];
    let m = &points[points.len() / 2];
    let l = &points[points.len()];

    if (f < m && m < l) || (l < m && m < f) {
        m
    } else if (m < f && f < l) || (l < f && f < m) {
        f
    } else {
        l
    }
}

fn quick_select_points_inner(mut points: &mut [Point], mut position: usize) {
    loop {
        let split = {
            let x = find_pivot(points).x;
            partition(points.iter_mut(), |p| p.x <= x)
        };

        match split.cmp(&position) {
            Ordering::Greater => {
                points = &mut { points }[0..split];
            }

            Ordering::Less => {
                position = position - split - 1;
                points = &mut { points }[(split + 1)..]
            }

            Ordering::Equal => {
                return;
            }
        }

        // if split == position {
        //     return;
        // }
        // else if split > position {
        //     points = &mut {points}[0..split];
        // }
        // else {
        //     position = position - split - 1;
        //     points = &mut {points}[(split + 1)..]
        // }
    }
}

pub fn quick_select_points(
    points: &mut [Point],
    position: usize,
) -> (&mut [Point], &mut Point, &mut [Point]) {
    quick_select_points_inner(points, position);

    let (left, right) = points.split_at_mut(position);
    let (pivot, right) = right.split_at_mut(1);
    let pivot = &mut pivot[0];
    (left, pivot, right)
}
