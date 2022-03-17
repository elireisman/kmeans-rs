use crate::point::Point;
use crate::render::render_iteration;
use std::collections::HashMap;

fn calc_centroid(points: &Vec<Point>) -> Point {
    let size = points.len() as f64;
    let x = points.iter().fold(0.0, |acc, v| acc + v.x) / size;
    let y = points.iter().fold(0.0, |acc, v| acc + v.y) / size;

    Point { x, y }
}

fn regroup_points(points: Vec<Point>, centroids: Vec<Point>) -> HashMap<Point, Vec<Point>> {
    let mut next = HashMap::new();

    for point in points {
        // copy centroid key and insert point into Vec of values
        let centroid = centroids
            .iter()
            .fold(None, |acc: Option<&Point>, candidate: &Point| match acc {
                None => Some(candidate),
                Some(current_best) => {
                    match candidate.sum_squared_error(&point)
                        < current_best.sum_squared_error(&point)
                    {
                        true => Some(candidate),
                        false => acc,
                    }
                }
            })
            .unwrap()
            .clone();

        next.entry(centroid).or_insert(vec![]).push(point);
    }

    next
}

// https://www.analyticsvidhya.com/blog/2019/08/comprehensive-guide-k-means-clustering/
pub fn execute(
    bounds: &(Point, Point),
    points: Vec<Point>,
    k: usize,
    iters: usize,
) -> HashMap<Point, Vec<Point>> {
    // validate inputs
    if points.len() <= k {
        panic!("kmeans: k param cannot be greater than the points vector size!");
    }
    if iters < 1 {
        panic!("kmeas: no point in performing less than 1 iteration!");
    }

    // initialize candidate centroids randomly
    let initial_centroids: Vec<Point> = Point::generate_points(&bounds, k);

    // perform the initial clustering using candidates
    let mut clusters = regroup_points(points, initial_centroids);

    // perform iterations
    for iter in 1..=iters {
        let mut next_centroids = vec![];
        for cluster in clusters.values() {
            let next_centroid = calc_centroid(cluster);
            next_centroids.push(next_centroid);
        }

        clusters = regroup_points(clusters.into_values().flatten().collect(), next_centroids);
        render_iteration(bounds, &clusters, k, iter).unwrap();
    }

    clusters
}
