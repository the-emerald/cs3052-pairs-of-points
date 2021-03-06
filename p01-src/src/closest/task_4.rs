use crate::geometry::{Point, PointPair, Distance};
use rand::prelude::*;
use std::collections::{HashMap};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MeshPoint {
    xth: isize,
    yth: isize
}

pub struct Mesh {
    size: f64,
    mesh: HashMap<MeshPoint, Vec<Point>>
}

impl Mesh {
    pub fn new(size: f64) -> Self {
        debug_assert!(size > 0.0);
        Self {
            size,
            mesh: HashMap::new()
        }
    }

    pub fn add_point_unchecked(&mut self, point: Point) {
        let point_mp = self.get_meshpoint_of_point(point);

        let v = self.mesh.entry(point_mp).or_insert(Vec::new());
        v.push(point);
    }

    // Adds points with consideration to neighbourhood dropping rules
    pub fn add_point(&mut self, point: Point) -> PointsInNeighbour {
        let point_mp = self.get_meshpoint_of_point(point);

        // If any of the neighbours contain a point
        let has_neighbour = self.neighbour_is_populated(point_mp);

        let v = self.mesh.entry(point_mp).or_insert(Vec::new());
        v.push(point);

        has_neighbour
    }

    pub fn closest_pair_to_point(&self, point: Point) -> Option<PointPair> {
        let point_mp = self.get_meshpoint_of_point(point);

        let closest = self.get_neighbours_of_mesh(point_mp).iter()
            .flat_map(|p| {
                self.mesh.get(p).cloned().unwrap_or_default()
            })
            .filter(|&p| p != point)
            .min_by(|a, b| a.distance_to(point).0.partial_cmp(&b.distance_to(point).0).unwrap());

        closest.map(|p| PointPair(point, p))
    }

    fn neighbour_is_populated(&mut self, point_mp: MeshPoint) -> PointsInNeighbour {
        if self.get_neighbours_of_mesh(point_mp)
            .iter()
            .map(|p| {
                self.mesh.get(p).cloned().unwrap_or_default()
            })
            .any(|hs| hs.len() > 1) {
            PointsInNeighbour::Yes
        } else {
            PointsInNeighbour::No
        }
    }

    fn get_meshpoint_of_point(&self, point: Point) -> MeshPoint {
        MeshPoint {
            xth: (point.x / self.size).floor() as isize,
            yth: (point.y / self.size).floor() as isize
        }
    }

    fn get_neighbours_of_mesh(&self, meshpoint: MeshPoint) -> [MeshPoint; 9] {
        let MeshPoint { xth: cx, yth: cy } = meshpoint;

        [
            MeshPoint { xth: cx-1, yth: cy+1 },
            MeshPoint { xth: cx, yth: cy+1 },
            MeshPoint { xth: cx+1, yth: cy+1 },

            MeshPoint { xth: cx-1, yth: cy },
            MeshPoint { xth: cx, yth: cy },
            MeshPoint { xth: cx+1, yth: cy },

            MeshPoint { xth: cx-1, yth: cy-1 },
            MeshPoint { xth: cx, yth: cy },
            MeshPoint { xth: cx+1, yth: cy+1 },
        ]
    }

}

pub enum PointsInNeighbour {
    Yes,
    No
}

#[derive(Clone, Debug)]
pub struct Task4 {
    points: Vec<Point>,
}

impl Task4 {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn find_closest_pair(&self) -> PointPair {
        // A Simple Randomized Sieve Algorithm for the Closest-Pair Problem. (Khuller, Matias 2009)
        let mut rng = StdRng::seed_from_u64(0x4749_4232_3050_4C53);
        let mut points_filtering = self.points.clone();

        // The filtering process
        let mut minimum;
        let mut random;
        loop {
            // Pick a random point from collection
            random = *(points_filtering.choose(&mut rng).unwrap());

            // Compute closest distance to all points, find minimum
            minimum = Task4::minimum_distance_to(&mut points_filtering, random);

            // Construct mesh with size minimum / 3
            let mut mesh = Mesh::new(minimum.0 / 3_f64);

            // Add points to mesh, remove points that are alone in their neighbourhood
            points_filtering = points_filtering
                .into_iter()
                .filter_map(|p| {
                    match mesh.add_point(p) {
                        PointsInNeighbour::Yes => Some(p),
                        PointsInNeighbour::No => None
                    }
                })
                .collect();

            // Stop when empty
            if points_filtering.len() == 0 {
                break;
            }
        }

        // Construct mesh of size minimum
        minimum = Task4::minimum_distance_to(&self.points, random);
        let mut mesh = Mesh::new(minimum.0);

        for point in &self.points {
            mesh.add_point_unchecked(*point);
        }

        self.points.iter()
            .filter_map(|p| mesh.closest_pair_to_point(*p))
            .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap()).unwrap()
    }

    fn minimum_distance_to(points: &[Point], point: Point) -> Distance {
        let min_point = points.iter()
            .filter(|&&p| p != point)
            .min_by(|a, b| a.distance_to(point).0.partial_cmp(&b.distance_to(point).0).unwrap()).unwrap();
        min_point.distance_to(point)
    }
}