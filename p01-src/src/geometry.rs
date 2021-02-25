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
