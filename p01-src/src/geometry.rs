use std::cmp::Ordering;
use std::ffi::CString;

/// Represents a distance between two points.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Distance(pub f64);

impl Distance {
    /// Returns the minimum of two distances.
    pub fn min(self, other: Distance) -> Distance {
        Distance(self.0.min(other.0))
    }

    /// Helper function to pretty-print a distance according to spec.
    /// # Safety
    /// The print has failed if a negative value is returned.
    pub unsafe fn pretty_print(self) -> i32 {
        let f = CString::new("%.9lg\n").unwrap();
        libc::printf(f.as_ptr(), self.0)
    }
}

/// Represents a single point in Euclidean 2D space.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Finds the distance to another point.
    pub fn distance_to(self, other: Point) -> Distance {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        Distance(((x_dist * x_dist) + (y_dist * y_dist)).sqrt())
    }
}

/// Represents a pair of points.
#[derive(Copy, Clone, Debug)]
pub struct PointPair(pub Point, pub Point);

impl PointPair {
    /// Returns the distance between the two points in the pair.
    pub fn distance(self) -> Distance {
        self.0.distance_to(self.1)
    }

    /// Returns the minimum of two `PointPair`s, defined by their distance.
    pub fn min(self, other: PointPair) -> PointPair {
        match self.distance().partial_cmp(&other.distance()).unwrap() {
            Ordering::Less => self,
            Ordering::Equal => self,
            Ordering::Greater => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_distance() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert_eq!(a.distance_to(b), Distance(5_f64))
    }

    #[test]
    fn points_distance_same() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = a;
        assert_eq!(a.distance_to(b), Distance(0_f64))
    }

    #[test]
    fn points_distance_negative() {
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: -3.0, y: -4.0 };
        assert_eq!(a.distance_to(b), Distance(5_f64))
    }
}
