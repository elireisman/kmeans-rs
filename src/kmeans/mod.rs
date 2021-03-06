use crate::cli::Config;
use crate::point::{generate_point, Centroid, Point};
use std::collections::HashMap;

pub type Cluster<'a> = HashMap<Centroid, Vec<&'a Point>>;

const EPSILON: f64 = 0.01;

// https://www.analyticsvidhya.com/blog/2019/08/comprehensive-guide-k-means-clustering/
pub fn execute<'a>(cfg: &Config, points: &'a Vec<Point>) -> Vec<Cluster<'a>> {
    // initialize candidate centroids randomly and assign cluster colors
    let initial_centroids = init_centroids(cfg);

    // perform the initial clustering using candidates
    let (mut clusters, mut total_error) = regroup_points(points, initial_centroids);

    // perform iterations
    let mut cache = vec![clusters.clone()];
    for iter in 1..=cfg.iterations {
        eprintln!("kmeans-rs: calculating iteration {}", iter);

        let mut next_centroids = vec![];
        for (centroid, cluster) in clusters {
            let next_centroid = calculate_next_centroid(centroid, cluster);
            next_centroids.push(next_centroid);
        }

        let prev_error = total_error;
        (clusters, total_error) = regroup_points(points, next_centroids);
        cache.push(clusters.clone());

        if f64::abs(prev_error - total_error) < EPSILON {
            eprintln!("kmeans-rs: converged at iteration {}", iter);
            return cache;
        }
    }

    cache
}

fn init_centroids(cfg: &Config) -> Vec<Centroid> {
    // initialize candidate centroids randomly and assign cluster colors
    (1..=cfg.k)
        .map(|color| Centroid {
            p: generate_point(cfg.bounds()),
            color: color,
        })
        .collect()
}

fn calculate_next_centroid(old_centroid: Centroid, cluster: Vec<&Point>) -> Centroid {
    let size = cluster.len() as f64;
    let x = cluster.iter().fold(0.0, |acc, v| acc + v.x) / size;
    let y = cluster.iter().fold(0.0, |acc, v| acc + v.y) / size;

    Centroid {
        p: Point { x: x, y: y },
        color: old_centroid.color,
    }
}

// compose a fresh mapping of input points to closest centroids.
// returns the mapping with total min error for the iteration
fn regroup_points(points: &Vec<Point>, centroids: Vec<Centroid>) -> (Cluster, f64) {
    let mut next = Cluster::new();
    let mut total_error = 0_f64;

    // group each point under best-fit centroid and capture the associated min error
    for point in points {
        let (centroid, min_error) = centroids.iter().fold(
            (None, f64::MAX),
            |acc: (Option<&Centroid>, f64), candidate: &Centroid| match acc {
                (None, _) => (Some(candidate), candidate.p.sum_squared_error(point)),
                (_, current_error) => {
                    let candidate_error = candidate.p.sum_squared_error(point);
                    match candidate_error < current_error {
                        true => (Some(candidate), candidate_error),
                        false => acc,
                    }
                }
            },
        );

        total_error += min_error;
        next.entry(centroid.unwrap().clone())
            .or_insert(vec![])
            .push(point);
    }

    (next, total_error)
}

#[cfg(test)]
mod test;
