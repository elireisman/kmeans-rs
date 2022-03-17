mod kmeans;
mod point;
mod render;

use clap::Parser;
use point::Point;
use std::collections::HashMap;

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
    let results: HashMap<String, Vec<Point>> = kmeans::execute(&bounds, points, k, iters)
        .into_iter()
        .map(|(k, v)| (format!("Centroid@{:?}", k), v))
        .collect();
    let json_result = serde_json::to_string(&results).unwrap();
    println!(
        "Success! Please enjoy this map of (centroid -> cluster):\n{}",
        json_result
    );
}
