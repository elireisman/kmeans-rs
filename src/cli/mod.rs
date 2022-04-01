use crate::point::{generate_clustered_points, Point};
use clap::Parser;
use serde_json::{from_reader, Value};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
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
        help = "maximum iterations to perform without convergence",
        default_value_t = 20
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
    pub points_file: Option<PathBuf>,

    #[clap(
        long,
        help = "path to directory where PNG images will be stored",
        default_value = "kmeans-pngs"
    )]
    pub png_out: PathBuf,

    #[clap(long, help = "render output as JSON")]
    pub json_out: bool,

    #[clap(long, help = "lower bound for points", default_value = "0,0")]
    pub lower_bound: Point,

    #[clap(long, help = "upper bound for points", default_value = "1000,1000")]
    pub upper_bound: Point,
}

impl Config {
    pub fn validate(&self) -> Result<(), Box<ValidationError>> {
        eprintln!("kmeans-rs: initialized with: {:?}", &self);

        if self.lower_bound.x >= self.upper_bound.x || self.lower_bound.y >= self.upper_bound.y {
            return Err(ValidationError::new(
                "kmeans-rs: lower bounds cannot be greater than upper bounds",
            ));
        }

        if self.k < 1 {
            return Err(ValidationError::new("kmeans-rs: k must be positive"));
        }

        if self.iterations < 1 {
            return Err(ValidationError::new(
                "kmean-rs: no point in performing less than 1 iteration",
            ));
        }

        if self.points_file.is_none() {
            if self.num_points < self.k {
                return Err(ValidationError::new(
                    "kmeans-rs: k is greater than num_points",
                ));
            }
        }

        Ok(())
    }

    pub fn bounds(&self) -> (&Point, &Point) {
        (&self.lower_bound, &self.upper_bound)
    }

    pub fn points(&self) -> Vec<Point> {
        if self.points_file.is_none() {
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

#[derive(Debug)]
pub struct ValidationError {
    err_msg: String,
}

impl ValidationError {
    fn new(msg: &str) -> Box<Self> {
        Box::new(Self {
            err_msg: msg.to_string(),
        })
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.err_msg)
    }
}

impl Error for ValidationError {
    fn description(&self) -> &str {
        &self.err_msg
    }
}
