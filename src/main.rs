mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    eprintln!("kmeans-rs: initialized with: {:?}", args);

    let points = point::Point::generate_points(args.bounds(), args.num_points);

    // convert results into JSON-friendly format, print it
    kmeans::execute(args, &points);
}
