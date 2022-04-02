use super::*;

fn test_cfg() -> Config {
    Config {
        k: 5_usize,
        iterations: 10_usize,
        num_points: 100_usize,
        points_file: None,
        png_out: PathBuf::from("/tmp/foobar"),
        json_out: true,
        lower_bound: Point { x: 0_f64, y: 0_f64 },
        upper_bound: Point {
            x: 20_f64,
            y: 20_f64,
        },
    }
}

#[test]
fn test_bounds() {
    let cfg = test_cfg();
    let resolved = cfg.bounds();
    assert_eq!(0_f64, resolved.0.x);
    assert_eq!(0_f64, resolved.0.y);
    assert_eq!(20_f64, resolved.1.x);
    assert_eq!(20_f64, resolved.1.y);
}

#[test]
fn test_points() {
    let cfg = test_cfg();
    let resolved = cfg.points();

    assert_eq!(cfg.num_points, resolved.len());
}

#[test]
fn test_validate_good_cfg() {
    let cfg = test_cfg();
    let result = cfg.validate();

    assert!(result.is_ok());
}

#[test]
fn test_validate_bounds() {
    let cfg = Config {
        lower_bound: Point {
            x: 300_f64,
            y: 300_f64,
        },
        ..test_cfg()
    };

    let result = cfg.validate();
    assert!(result.is_err());
}

#[test]
fn test_validate_k_gte_num_points() {
    let cfg = Config {
        k: 44,
        num_points: 43,
        ..test_cfg()
    };

    let result = cfg.validate();
    assert!(result.is_err());
}

#[test]
fn test_validate_k() {
    let cfg = Config { k: 0, ..test_cfg() };

    let result = cfg.validate();
    assert!(result.is_err());
}
#[test]
fn test_validate_iterations() {
    let cfg = Config {
        iterations: 0,
        ..test_cfg()
    };

    let result = cfg.validate();
    assert!(result.is_err());
}
