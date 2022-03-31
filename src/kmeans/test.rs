use super::*;

#[test]
fn test_calculate_next_centroid() {
    let prev = Centroid {
        p: Point { x: 0_f64, y: 0_f64 },
        color: Some(5),
    };

    let points = vec![
        &Point { x: 0_f64, y: 0_f64 },
        &Point {
            x: 20_f64,
            y: 40_f64,
        },
    ];

    let next = calculate_next_centroid(&prev, &points);
    assert_eq!(prev.color, next.color);
    assert_eq!(10_f64, next.p.x);
    assert_eq!(20_f64, next.p.y);
}
