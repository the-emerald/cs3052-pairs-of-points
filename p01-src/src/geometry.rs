#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Distance(pub f64);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(self, other: Point) -> Distance {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        Distance(((x_dist * x_dist) + (y_dist * y_dist)).sqrt())
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
