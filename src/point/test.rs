use super::*;

#[test]
fn test_gen_points() {
    let bounds = (
        &Point { x: 0_f64, y: 0_f64 },
        &Point {
            x: 100_f64,
            y: 100_f64,
        },
    );
    let ps = generate_points(bounds, 13);

    assert_eq!(13, ps.len());
    ps.iter().for_each(|p| {
        assert!(p.x >= bounds.0.x);
        assert!(p.y >= bounds.0.y);
        assert!(p.x < bounds.1.x);
        assert!(p.y < bounds.1.y);
    });
}

#[test]
fn test_gen_point() {
    let bounds = (
        &Point { x: 0_f64, y: 0_f64 },
        &Point {
            x: 100_f64,
            y: 100_f64,
        },
    );
    let p = generate_point(bounds);

    assert!(p.x >= bounds.0.x);
    assert!(p.y >= bounds.0.y);
    assert!(p.x < bounds.1.x);
    assert!(p.y < bounds.1.y);
}

#[test]
fn test_point_from_str() {
    let result = Point::from_str("22,33");
    assert!(&result.is_ok());

    let p: Point = result.unwrap();
    assert_eq!(22_f64, p.x);
    assert_eq!(33_f64, p.y);
}

#[test]
fn test_eq_point() {
    let p1 = Point { x: 2_f64, y: 4_f64 };
    let p2 = p1.clone();
    let p3 = Point {
        x: 111.1_f64,
        y: 5555.5_f64,
    };

    assert_eq!(p1, p2);
    assert!(p1 != p3);
}
