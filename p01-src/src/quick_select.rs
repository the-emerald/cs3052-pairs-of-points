use crate::geometry::Point;
use itertools::partition;
use std::cmp::Ordering;

fn find_pivot(points: &[Point]) -> usize {
    let f = &points[0];
    let m = &points[(points.len() - 1) / 2];
    let l = &points[points.len() - 1];

    if (f < m && m < l) || (l < m && m < f) {
        // Middle
        (points.len() - 1) / 2
    } else if (m < f && f < l) || (l < f && f < m) {
        // First
        0
    } else {
        // Last
        points.len() - 1
    }
}

fn quick_select_points_inner(mut points: &mut [Point], mut position: usize) {
    loop {
        let split = {
            let pivot = find_pivot(points);
            let pivot_x = points[pivot].x;
            points.swap(pivot, points.len() - 1);
            let s = partition(points.iter_mut(), |p| p.x < pivot_x);
            points.swap(s, points.len() - 1);
            s
        };

        match split.cmp(&position) {
            Ordering::Equal => {
                return;
            }
            Ordering::Greater => {
                points = &mut { points }[0..split];
            }
            Ordering::Less => {
                position = position - split - 1;
                points = &mut { points }[(split + 1)..]
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_select_small() {
        let mut points = vec![
            Point { x: -5.0, y: 0.0 },
            Point { x: -4.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: -3.0, y: 0.0 },
            Point { x: 2.0, y: 0.0 },
        ];
        let length = points.len();

        // Clone points and use a known-good implementation
        let mut points2 = points.clone();
        let (_, median, _) =
            points2.select_nth_unstable_by(length / 2, |a, b| a.x.partial_cmp(&b.x).unwrap());

        let (lesser, pivot, greater) = quick_select_points(&mut points[..], length / 2);

        assert!(lesser.iter().all(|x| x <= pivot));
        assert!(greater.iter().all(|x| x > pivot));
        assert_eq!(*median, *pivot)
    }

    #[test]
    fn quick_select_many() {
        let mut points = vec![
            Point { x: -5.0, y: 0.0 },
            Point { x: -4.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: -3.0, y: 0.0 },
            Point { x: 2.0, y: 0.0 },
            Point { x: 19.0, y: 0.0 },
            Point { x: -2.0, y: 0.0 },
            Point { x: -1298.0, y: 0.0 },
            Point { x: 8.0, y: 0.0 },
            Point { x: 3.0, y: 0.0 },
            Point { x: 12.0, y: 0.0 },
            Point { x: -2.0, y: 0.0 },
            Point { x: -1.0, y: 0.0 },
        ];

        let length = points.len();

        // Clone points and use a known-good implementation
        let mut points2 = points.clone();
        let (_, median, _) =
            points2.select_nth_unstable_by(length / 2, |a, b| a.x.partial_cmp(&b.x).unwrap());

        let (lesser, pivot, greater) = quick_select_points(&mut points[..], length / 2);

        assert!(lesser.iter().all(|x| x <= pivot));
        assert!(greater.iter().all(|x| x > pivot));
        assert_eq!(*median, *pivot)
    }
}
