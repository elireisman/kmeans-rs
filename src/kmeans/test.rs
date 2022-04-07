use super::*;
use std::collections::HashSet;
use std::path::PathBuf;

#[test]
fn test_calculate_next_centroid() {
    let prev = Centroid {
        p: Point { x: 0_f64, y: 0_f64 },
        color: 5,
    };

    let points = vec![
        &Point { x: 0_f64, y: 0_f64 },
        &Point {
            x: 20_f64,
            y: 40_f64,
        },
    ];

    let next = calculate_next_centroid(prev, points);
    assert_eq!(5, next.color);
    assert_eq!(10_f64, next.p.x);
    assert_eq!(20_f64, next.p.y);
}

#[test]
fn test_init_centroid() {
    let cfg = &Config {
        k: 13,
        num_points: 100,
        iterations: 10,
        png_out: PathBuf::from("test"),
        points_file: None,
        json_out: true,
        lower_bound: Point { x: 0_f64, y: 0_f64 },
        upper_bound: Point {
            x: 100_f64,
            y: 100_f64,
        },
    };
    let centroids = init_centroids(cfg);

    assert_eq!(13, centroids.len());

    let mut colors_seen = HashSet::new();
    centroids.iter().for_each(|c| {
        assert!(c.color > 0_usize);
        assert!(!colors_seen.contains(&c.color));
        colors_seen.insert(c.color);
    });
}
