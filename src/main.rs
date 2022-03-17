mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    eprintln!("kmeans-rs: initialized with: {:?}", args);

    let points = point::Point::generate_points(args.bounds(), args.num_points);
    let result = kmeans::execute(&args, &points);

    eprintln!("kmeans-rs: rendering output");
    if args.json_out {
        render::render_json(&result);
    }

    if !args.png_out.is_empty() {
        std::fs::create_dir_all(&args.png_out).unwrap();
        for (iteration, clusters) in result.iter().enumerate() {
            render::render_iteration_png(&args, clusters, iteration).unwrap();
        }
    }
}
