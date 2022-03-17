mod point;
mod render;

use clap::Parser;
use point::Point;
use std::collections::HashMap;

fn calc_centroid(points: &Vec<Point>) -> Point {
    let size = points.len() as f64;
    let x = points.iter().fold(0.0, |acc, v| acc + v.x) / size;
    let y = points.iter().fold(0.0, |acc, v| acc + v.y) / size;

    Point { x, y }
}

fn regroup_points(points: Vec<Point>, centroids: Vec<Point>) -> HashMap<Point, Vec<Point>> {
    let mut next = HashMap::new();

    for point in points {
        // copy centroid key and insert point into Vec of values
        let centroid = centroids
            .iter()
            .fold(None, |acc: Option<&Point>, candidate: &Point| match acc {
                None => Some(candidate),
                Some(current_best) => {
                    match candidate.sum_squared_error(&point)
                        < current_best.sum_squared_error(&point)
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
fn kmeans(
    bounds: &(Point, Point),
    points: Vec<Point>,
    k: usize,
    iters: usize,
) -> HashMap<Point, Vec<Point>> {
    // validate inputs
    if points.len() <= k {
        panic!("kmeans: k param cannot be greater than the points vector size!");
    }
    if iters < 1 {
        panic!("kmeas: no point in performing less than 1 iteration!");
    }

    // initialize candidate centroids randomly
    let initial_centroids: Vec<Point> = Point::generate_points(&bounds, k);

    // perform the initial clustering using candidates
    let mut clusters = regroup_points(points, initial_centroids);

    // perform iterations
    for iter in 1..=iters {
        let mut next_centroids = vec![];
        for cluster in clusters.values() {
            let next_centroid = calc_centroid(cluster);
            next_centroids.push(next_centroid);
        }

        clusters = regroup_points(clusters.into_values().flatten().collect(), next_centroids);
        render::render_iteration(bounds, &clusters, k, iter).unwrap();
    }

    clusters
}

// toy K-Means clustering on random set of input points. renders a PNG per iteration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 4)]
    k: usize,

    #[clap(short, long, default_value_t = 12)]
    iterations: usize,

    #[clap(short, long, default_value_t = 100)]
    num_points: usize,
}

fn main() {
    let args = Args::parse();
    let k = args.k;
    let iters = args.iterations;
    let num_points = args.num_points;
    println!(
        "Inputs: k({}) iterations({}) number_of_points({})",
        k, iters, num_points
    );

    let bounds = (
        Point { x: 0.0, y: 0.0 },
        Point {
            x: 1000.0,
            y: 1000.0,
        },
    );
    let points = Point::generate_points(&bounds, num_points);

    // convert results into JSON-friendly format, print it
    let results: HashMap<String, Vec<Point>> = kmeans(&bounds, points, k, iters)
        .into_iter()
        .map(|(k, v)| (format!("Centroid@{:?}", k), v))
        .collect();
    let json_result = serde_json::to_string(&results).unwrap();
    println!(
        "Success! Please enjoy this map of (centroid -> cluster):\n{}",
        json_result
    );
}