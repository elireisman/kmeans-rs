mod kmeans;
mod point;
mod render;

use clap::Parser;
use point::Point;

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
        Point {
            x: 0.0,
            y: 0.0,
            color: None,
        },
        Point {
            x: 1000.0,
            y: 1000.0,
            color: None,
        },
    );
    let points = Point::generate_points(&bounds, num_points);

    // convert results into JSON-friendly format, print it
    kmeans::execute(&bounds, &points, k, iters);
}
