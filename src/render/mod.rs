use crate::point::Point;
use plotters::prelude::*;
use std::collections::HashMap;

pub fn render_iteration(
    bounds: &(Point, Point),
    clusters: &HashMap<Point, Vec<Point>>,
    k: usize,
    iter: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("kmeans-iter-{:03.1}.png", iter);
    let root = BitMapBackend::new(&filename, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("K-means (k={}, iteration={})", k, iter),
            ("sans-serif", 50).into_font(),
        )
        .margin(5 as u32)
        .x_label_area_size(30 as u32)
        .y_label_area_size(30 as u32)
        .build_cartesian_2d((bounds.0.x)..(bounds.1.x), (bounds.0.y)..(bounds.1.y))?;
    chart.configure_mesh().draw()?;

    let mut color_iter = 1;
    for points in clusters.values() {
        chart.draw_series(PointSeries::of_element(
            points.iter().map(|p| (p.x, p.y)),
            5,
            &Palette99::pick(color_iter),
            &|c, s: u32, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;
        color_iter += 1;
    }

    chart.draw_series(PointSeries::of_element(
        clusters.keys().map(|p| (p.x, p.y)),
        5,
        &BLACK,
        &|c, s: u32, st| {
            return EmptyElement::at(c)
                + Circle::new((0, 0), s, st.filled())
                + Text::new(
                    format!("({:.1}, {:.1})", c.0, c.1),
                    (10, 0),
                    ("sans-serif", 12).into_font(),
                );
        },
    ))?;

    Ok(())
}
