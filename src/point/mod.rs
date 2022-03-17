use rand::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::ParseFloatError;
use std::str::FromStr;

pub fn generate_points(bounds: (&Point, &Point), cardinality: usize) -> Vec<Point> {
    let mut points = vec![];
    for _ in 1..=cardinality {
        points.push(generate_point(bounds, None));
    }

    points
}

pub fn generate_point(bounds: (&Point, &Point), color: Option<usize>) -> Point {
    let mut r = rand::thread_rng();

    loop {
        let p = Point {
            x: r.gen::<f64>() * bounds.1.x,
            y: r.gen::<f64>() * bounds.1.y,
            color: color,
        };

        return match &p {
            Point { x, y, .. }
                if *x >= bounds.0.x && *x < bounds.1.x && *y >= bounds.0.y && *y < bounds.1.y =>
            {
                p
            }
            _ => generate_point(bounds, color),
        };
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
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

impl FromStr for Point {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

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
