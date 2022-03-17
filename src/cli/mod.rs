use crate::point::Point;
use clap::Parser;

// toy K-Means clustering on random set of input points. renders a PNG per iteration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // K param (number of clusters)
    #[clap(short, long, default_value_t = 4)]
    pub k: usize,

    // number of iterations to run
    #[clap(short, long, default_value_t = 12)]
    pub iterations: usize,

    // number of (randomly generated) input points to partition
    #[clap(short, long, default_value_t = 100)]
    pub num_points: usize,

    // per-iteration PNGs will be rendered and stored in this directory
    #[clap(short, long, default_value = "kmeans-pngs")]
    pub png_out: String,

    // render per-iteration JSON output
    #[clap(short, long)]
    pub json_out: bool,

    // lower bound
    #[clap(long, default_value = "(0,0)")]
    pub lower_bound: Point,

    // upper bound
    #[clap(long, default_value = "(1000,1000)")]
    pub upper_bound: Point,
}

impl Args {
    pub fn bounds(&self) -> (&Point, &Point) {
        (&self.lower_bound, &self.upper_bound)
    }
}
