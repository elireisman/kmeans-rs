use crate::point::{generate_points, Point};
use clap::Parser;
use serde_json::{from_reader, Value};
use std::fs::File;
use std::io::BufReader;

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

    // if present, the path to a file containing a JSON array of points of the form
    // [{"x": 1.1, "y": 2.2}, {"x": 3.3, "y": 4.4}, ...] to use as inputs.
    // Overrides --num-points if present.
    #[clap(short, long)]
    pub points_file: Option<String>,

    // render and store per-iteration PNG images
    #[clap(long, default_value = "kmeans-pngs")]
    pub png_out: String,

    // render per-iteration JSON output
    #[clap(long)]
    pub json_out: bool,

    // points lower bound
    #[clap(long, default_value = "(0,0)")]
    pub lower_bound: Point,

    // points upper bound
    #[clap(long, default_value = "(1000,1000)")]
    pub upper_bound: Point,
}

impl Args {
    pub fn bounds(&self) -> (&Point, &Point) {
        (&self.lower_bound, &self.upper_bound)
    }

    pub fn points(&self) -> Vec<Point> {
        if self.points_file.is_none() {
            if self.num_points < self.k {
                panic!("kmeans-rs: 'k' greater than 'num_points'");
            }

            return generate_points(self.bounds(), self.num_points);
        }

        let file = File::open(self.points_file.as_ref().unwrap()).unwrap();
        let reader = BufReader::new(file);
        let input: Vec<Value> = from_reader(reader).unwrap();

        if input.len() < self.k {
            panic!("kmeans-rs: 'k' greater than points found in input file");
        }

        return input
            .iter()
            .map(|json_val: &Value| {
                let p = json_val.as_object().unwrap();

                Point {
                    x: p["x"].as_f64().unwrap(),
                    y: p["y"].as_f64().unwrap(),
                    color: None,
                }
            })
            .collect();
    }
}
