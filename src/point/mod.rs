use rand::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::f64::consts;
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::str::FromStr;

#[allow(dead_code)]
pub fn generate_points(bounds: (&Point, &Point), cardinality: usize) -> Vec<Point> {
    let mut points = vec![];
    for _ in 1..=cardinality {
        points.push(generate_point(bounds, None));
    }

    points
}

pub fn generate_clustered_points(
    bounds: (&Point, &Point),
    k: usize,
    cardinality: usize,
) -> Vec<Point> {
    let centers = generate_points(bounds, k);
    let min_bound = match (
        (bounds.1.x - bounds.0.x) as f64,
        (bounds.1.x - bounds.0.y) as f64,
    ) {
        (x, y) if x <= y => x,
        (x, y) if x > y => y,
        _ => panic!("wtf min_bound!?"),
    };

    let max_radius = min_bound / (k as f64);

    let mut points = vec![];
    for ndx in 0..cardinality {
        let selection = ndx % centers.len();
        points.push(generate_clustered_point(
            bounds,
            centers.get(selection).unwrap(),
            max_radius,
            None,
        ));
    }

    points
}

fn generate_clustered_point(
    bounds: (&Point, &Point),
    cluster_center: &Point,
    radius: f64,
    color: Option<usize>,
) -> Point {
    let mut r = rand::thread_rng();
    let dist = r.gen_range((-radius)..radius);
    let angle = r.gen_range(0.0_f64..(2.0_f64 * consts::PI));

    let candidate = Point {
        x: cluster_center.x + (dist * angle.sin()),
        y: cluster_center.y + (dist * angle.cos()),
        color: color,
    };

    match candidate {
        Point { ref x, ref y, .. }
            if *x >= bounds.0.x && *x < bounds.1.x && *y >= bounds.0.y && *y < bounds.1.y =>
        {
            candidate
        }
        _ => generate_clustered_point(bounds, cluster_center, radius, color),
    }
}

pub fn generate_point(bounds: (&Point, &Point), color: Option<usize>) -> Point {
    let mut r = rand::thread_rng();
    let x: f64 = r.gen_range(bounds.0.x..bounds.1.x);
    let y: f64 = r.gen_range(bounds.0.y..bounds.1.y);

    Point {
        x: x,
        y: y,
        color: color,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,

    #[serde(skip_serializing)]
    pub color: Option<usize>,
}

const EPSILON: f64 = 0.00001;

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let diffx = f64::abs(self.x - other.x);
        let diffy = f64::abs(self.y - other.y);

        return diffx < EPSILON && diffy < EPSILON && self.color == other.color;
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            std::mem::transmute::<f64, u64>(self.x).hash(state);
            std::mem::transmute::<f64, u64>(self.y).hash(state);
        }
        if let Some(color) = self.color {
            color.hash(state);
        }
    }
}

// convert raw String input of the form "11,22.3" into Point
impl FromStr for Point {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x_fromstr = coords[0].parse::<f64>()?;
        let y_fromstr = coords[1].parse::<f64>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
            color: None,
        })
    }
}

impl Point {
    pub fn sum_squared_error(&self, other: &Self) -> f64 {
        f64::powf(self.x - other.x, 2.0) + f64::powf(self.y - other.y, 2.0)
    }

    #[allow(dead_code)]
    pub fn select_initial_centroids(points: &Vec<Point>, k: usize) -> Vec<Point> {
        let mut r = rand::thread_rng();

        let mut selections = HashMap::new();
        for _ in 0..k {
            let candidate = r.gen::<usize>() % points.len();
            if selections.get(&candidate).is_some() {
                continue;
            }
            let selected = points[candidate].clone();
            selections.insert(candidate, selected);
        }

        selections.into_values().collect()
    }
}
