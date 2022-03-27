mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    if let Err(e) = args.validate() {
        panic!("{}", e);
    }
    let points = args.points();
    let result = kmeans::execute(&args, &points);

    eprintln!("kmeans-rs: rendering output");
    if args.json_out {
        let output = render::json_all_iterations(&result).unwrap();
        println!("{}", output);
    }

    if !args.png_out.is_empty() {
        let _ = std::fs::remove_dir_all(&args.png_out);
        std::fs::create_dir_all(&args.png_out).unwrap();
        render::png_all_iterations(&args, &result).unwrap();
    }
}
