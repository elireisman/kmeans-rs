mod cli;
mod kmeans;
mod point;
mod render;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    eprintln!("kmeans-rs: initialized with: {:?}", &args);

    if args.iterations < 1 {
        panic!("kmean-rs: no point in performing less than 1 iteration");
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
