mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    // parse and validate CLI args
    let cfg = cli::Config::parse();
    if let Err(e) = cfg.validate() {
        panic!("{}", e);
    }

    // generate or load 2D input points from file
    let points = cfg.points();
    if let Err(e) = points {
        panic!("{}", e);
    }
    let points = points.unwrap();

    // run K-means on the inputs
    let result = kmeans::execute(&cfg, &points);

    // render outputs depending on CLI args
    eprintln!("kmeans-rs: rendering output");
    if cfg.json_out {
        let output = render::json_all_iterations(&result).unwrap();
        println!("{}", output);
    }

    let _ = std::fs::remove_dir_all(&cfg.png_out);
    std::fs::create_dir_all(&cfg.png_out).unwrap();
    render::png_all_iterations(&cfg, &result).unwrap();
}
