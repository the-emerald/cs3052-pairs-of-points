use crate::geometry::{Distance, Point, PointPair};
use fnv::FnvHashMap;
use rand::prelude::*;

/// A single mesh, indexed by its position along the `x` and `y` axis.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MeshPoint {
    xth: isize,
    yth: isize,
}

/// A collection of meshes with a defined edge-to-edge size.
///
/// Internally, this is represented by a HashMap of x and y index to a vector of points. When an
/// item is added into the collection, it is placed into the correct 'bucket' by finding out its
/// closest index.
pub struct Mesh {
    size: f64,
    mesh: FnvHashMap<MeshPoint, Vec<Point>>,
}

impl Mesh {
    /// Create a new mesh with a specified size.
    /// # Panics
    /// Function will panic if the size is zero.
    pub fn new(size: f64) -> Self {
        debug_assert!(size > 0.0);
        Self {
            size,
            mesh: FnvHashMap::default(),
        }
    }

    /// Add a point to the mesh, but do not check if the point has any neighbours.
    pub fn add_point_unchecked(&mut self, point: Point) {
        let point_mp = self.get_meshpoint_of_point(point);

        let v = self.mesh.entry(point_mp).or_insert_with(Vec::new);
        v.push(point);
    }

    /// Add a point to the mesh and returns whether the point added had any neighbours. This function
    /// involves more lookups than the unchecked variant and requires more computation.
    pub fn add_point(&mut self, point: Point) -> PointsInNeighbour {
        let point_mp = self.get_meshpoint_of_point(point);

        // If any of the neighbours contain a point
        let has_neighbour = self.neighbour_is_populated(point_mp);

        let v = self.mesh.entry(point_mp).or_insert_with(Vec::new);
        v.push(point);

        has_neighbour
    }

    /// Finds the closest pair to the point given in its neighbourhood.
    /// # Returns
    /// Function returns `None` if it is the only point in its neighbourhood.
    pub fn closest_pair_to_point_in_neighbour(&self, point: Point) -> Option<PointPair> {
        let point_mp = self.get_meshpoint_of_point(point);

        let closest = self
            .get_neighbours_of_mesh(point_mp)
            .iter()
            .flat_map(|p| self.mesh.get(p))
            .flatten()
            .filter(|p| **p != point)
            .min_by(|a, b| {
                a.distance_to(point)
                    .0
                    .partial_cmp(&b.distance_to(point).0)
                    .unwrap()
            });

        closest.map(|p| PointPair(point, *p))
    }

    /// Checks whether a neighbourhood is populated, given a single mesh.
    fn neighbour_is_populated(&self, point_mp: MeshPoint) -> PointsInNeighbour {
        if self
            .get_neighbours_of_mesh(point_mp)
            .iter()
            .flat_map(|p| self.mesh.get(p))
            .any(|hs| hs.len() > 1)
        {
            PointsInNeighbour::Yes
        } else {
            PointsInNeighbour::No
        }
    }

    /// Returns the mesh a point belongs to.
    fn get_meshpoint_of_point(&self, point: Point) -> MeshPoint {
        MeshPoint {
            xth: (point.x / self.size).floor() as isize,
            yth: (point.y / self.size).floor() as isize,
        }
    }

    /// Returns the neighbours of a mesh.
    fn get_neighbours_of_mesh(&self, meshpoint: MeshPoint) -> [MeshPoint; 9] {
        let MeshPoint { xth: cx, yth: cy } = meshpoint;

        [
            MeshPoint {
                xth: cx - 1,
                yth: cy + 1,
            },
            MeshPoint {
                xth: cx,
                yth: cy + 1,
            },
            MeshPoint {
                xth: cx + 1,
                yth: cy + 1,
            },
            MeshPoint {
                xth: cx - 1,
                yth: cy,
            },
            MeshPoint { xth: cx, yth: cy },
            MeshPoint {
                xth: cx + 1,
                yth: cy,
            },
            MeshPoint {
                xth: cx - 1,
                yth: cy - 1,
            },
            MeshPoint {
                xth: cx,
                yth: cy - 1,
            },
            MeshPoint {
                xth: cx + 1,
                yth: cy - 1,
            },
        ]
    }
}

/// Whether a point had any neighbour
pub enum PointsInNeighbour {
    /// Yes
    Yes,
    /// No (it was alone)
    No,
}

/// Task 3 (1): A randomised algorithm as devised by Khuller, Matias 2009.
#[derive(Clone, Debug)]
pub struct Task4 {
    points: Vec<Point>,
}

impl Task4 {
    /// Create a new `Task4` struct.
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    /// Find the closest pair of points in the list.
    pub fn find_closest_pair(&self) -> PointPair {
        // A Simple Randomized Sieve Algorithm for the Closest-Pair Problem. (Khuller, Matias 2009)
        let mut rng = StdRng::seed_from_u64(0x4749_4232_3050_4C53);
        let mut points_filtering = self.points.clone();

        // Filtering
        let mut minimum;
        let mut random;
        loop {
            // Pick a random point from collection
            random = *(points_filtering.choose(&mut rng).unwrap());

            // Compute closest distance to all points, find minimum
            minimum = Task4::minimum_distance_to(&points_filtering, random);

            // Construct mesh with size minimum / 3
            let mut mesh = Mesh::new(minimum.0 / 3_f64);

            // Add points to mesh, remove points that are alone in their neighbourhood
            points_filtering = points_filtering
                .into_iter()
                .filter(|p| match mesh.add_point(*p) {
                    PointsInNeighbour::Yes => true,
                    PointsInNeighbour::No => false,
                })
                .collect();

            // Stop when empty
            if points_filtering.len() < 2 {
                break;
            }
        }

        // Construct mesh of size minimum
        minimum = Task4::minimum_distance_to(&self.points, random);
        let mut mesh = Mesh::new(minimum.0);

        for point in &self.points {
            mesh.add_point_unchecked(*point);
        }

        self.points
            .iter()
            .filter_map(|p| mesh.closest_pair_to_point_in_neighbour(*p))
            .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }

    fn minimum_distance_to(points: &[Point], point: Point) -> Distance {
        let min_point = points
            .iter()
            .filter(|&&p| p != point)
            .min_by(|a, b| {
                a.distance_to(point)
                    .0
                    .partial_cmp(&b.distance_to(point).0)
                    .unwrap()
            })
            .unwrap();
        min_point.distance_to(point)
    }
}
