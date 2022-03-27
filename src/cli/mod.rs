use crate::point::{generate_clustered_points, Point};
use clap::Parser;
use serde_json::{from_reader, Value};
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        help = "K param (number of clusters)",
        default_value_t = 4
    )]
    pub k: usize,

    #[clap(
        short,
        long,
        help = "number of iterations to perform",
        default_value_t = 12
    )]
    pub iterations: usize,

    #[clap(
        short,
        long,
        help = "number of (randomly generated) input points to cluster",
        default_value_t = 100
    )]
    pub num_points: usize,

    #[clap(
        short,
        long,
        help = "path to a JSON file containing input points of the form [{\"x\": 1.1, \"y\": 2.2}, {\"x\": 3.3, \"y\": 4.4}, ...]"
    )]
    pub points_file: Option<String>,

    #[clap(
        long,
        help = "path to directory where per-iteration PNG images will be stored",
        default_value = "kmeans-pngs"
    )]
    pub png_out: String,

    #[clap(long, help = "render per-iteration JSON output")]
    pub json_out: bool,

    #[clap(long, help = "lower bound for points", default_value = "0,0")]
    pub lower_bound: Point,

    #[clap(long, help = "higher bound for points", default_value = "1000,1000")]
    pub upper_bound: Point,
}

impl Args {
    pub fn bounds(&self) -> (&Point, &Point) {
        if self.lower_bound.x >= self.upper_bound.x || self.lower_bound.y >= self.upper_bound.y {
            panic!("kmeans-rs: lower bounds cannot be greater than upper bounds");
        }

        (&self.lower_bound, &self.upper_bound)
    }

    pub fn points(&self) -> Vec<Point> {
        if self.points_file.is_none() {
            if self.num_points < self.k {
                panic!("kmeans-rs: 'k' greater than 'num_points'");
            }

            return generate_clustered_points(self.bounds(), self.k, self.num_points);
        }

        let file = File::open(self.points_file.as_ref().unwrap()).unwrap();
        let reader = BufReader::new(file);
        let input: Vec<Value> = from_reader(reader).unwrap();

        if input.len() < self.k {
            panic!("kmeans-rs: 'k' greater than points found in input file");
        }

        let points: Vec<Point> = input
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

        let bounds = self.bounds();
        if points
            .iter()
            .any(|p| p.x < bounds.0.x || p.x >= bounds.1.x || p.y < bounds.0.y || p.y >= bounds.1.y)
        {
            panic!(
                "kmeans-rs: some input points are out of bounds: {:?}",
                bounds
            );
        }

        points
    }
}
