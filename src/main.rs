mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    let cfg = cli::Config::parse();
    if let Err(e) = cfg.validate() {
        panic!("{}", e);
    }
    let points = cfg.points();
    let result = kmeans::execute(&cfg, &points);

    eprintln!("kmeans-rs: rendering output");
    if cfg.json_out {
        let output = render::json_all_iterations(&result).unwrap();
        println!("{}", output);
    }

    if !cfg.png_out.is_empty() {
        let _ = std::fs::remove_dir_all(&cfg.png_out);
        std::fs::create_dir_all(&cfg.png_out).unwrap();
        render::png_all_iterations(&cfg, &result).unwrap();
    }
}
