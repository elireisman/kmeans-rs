use crate::cli::Args;
use crate::point::Point;
use crate::render::{render_iteration_json, render_iteration_png};
use std::collections::HashMap;

pub type Cluster<'a> = HashMap<Point, Vec<&'a Point>>;

fn calculate_next_centroid(old_centroid: &Point, points: &Vec<&Point>) -> Point {
    let size = points.len() as f64;
    let x = points.iter().fold(0.0, |acc, v| acc + v.x) / size;
    let y = points.iter().fold(0.0, |acc, v| acc + v.y) / size;

    Point {
        x: x,
        y: y,
        color: old_centroid.color,
    }
}

fn regroup_points(points: &Vec<Point>, centroids: Vec<Point>) -> Cluster {
    let mut next = Cluster::new();

    for point in points {
        // copy centroid key and insert point into Vec of values
        let centroid = centroids
            .iter()
            .fold(None, |acc: Option<&Point>, candidate: &Point| match acc {
                None => Some(candidate),
                Some(current_best) => {
                    match candidate.sum_squared_error(point) < current_best.sum_squared_error(point)
                    {
                        true => Some(candidate),
                        false => acc,
                    }
                }
            })
            .unwrap()
            .clone();

        next.entry(centroid).or_insert(vec![]).push(point);
    }

    next
}

// https://www.analyticsvidhya.com/blog/2019/08/comprehensive-guide-k-means-clustering/
pub fn execute(args: Args, points: &Vec<Point>) {
    // validate inputs
    if points.len() <= args.k {
        panic!("kmeans-rs: k param cannot be greater than the points vector size!");
    }
    if args.iterations < 1 {
        panic!("kmean-rs: no point in performing less than 1 iteration!");
    }

    // initialize candidate centroids randomly and assign cluster colors
    let initial_centroids = (1..=args.k)
        .map(|color| Point::generate_point(args.bounds(), Some(color)))
        .collect();

    // perform the initial clustering using candidates
    let mut clusters = regroup_points(points, initial_centroids);

    // perform iterations
    //let mut cache = vec![clusters.clone()];
    for iter in 1..=args.iterations {
        let mut next_centroids = vec![];
        for (centroid, cluster) in &clusters {
            let next_centroid = calculate_next_centroid(centroid, cluster);
            next_centroids.push(next_centroid);
        }
        clusters = regroup_points(points, next_centroids);

        render_iteration_json(iter, &clusters);
        render_iteration_png(args.bounds(), &clusters, args.k, iter).unwrap();
    }
}
