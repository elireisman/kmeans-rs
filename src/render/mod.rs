use crate::cli::Config;
use crate::kmeans::Cluster;
use crate::point::{Centroid, Point};
use plotters::prelude::*;
use rayon::prelude::*;
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::mpsc::channel;

#[derive(Serialize)]
struct ClusterJson<'a> {
    centroid: &'a Centroid,
    cluster: &'a Vec<&'a Point>,
}

#[derive(Serialize)]
struct IterationJson<'a> {
    iteration: usize,
    clusters: Vec<ClusterJson<'a>>,
}

// render JSON output for all iterations of K-means performed
pub fn json_all_iterations(all_clusters: &Vec<Cluster>) -> Result<String, Box<dyn Error>> {
    let mut result = vec![];
    for (iteration, clusters) in all_clusters.iter().enumerate() {
        let formatted = IterationJson {
            iteration: iteration,
            clusters: clusters
                .iter()
                .map(|(k, v)| ClusterJson {
                    centroid: k,
                    cluster: v,
                })
                .collect(),
        };

        result.push(formatted);
    }

    let rendered = serde_json::to_string(&result)?;
    Ok(rendered)
}

// render PNG for all iterations of K-means
pub fn png_all_iterations(
    cfg: &Config,
    all_clusters: &Vec<Cluster>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (sender, receiver) = channel();

    (0_usize..all_clusters.len())
        .into_par_iter()
        .for_each_with(sender, |s, iter| {
            if let Err(e) = png_for_iteration(cfg, all_clusters.get(iter).unwrap(), iter) {
                s.send(e).unwrap()
            }
        });

    for render_err in receiver {
        return Err(render_err);
    }

    Ok(())
}

// render PNG for a single K-means iteration
fn png_for_iteration(
    cfg: &Config,
    clusters: &Cluster,
    iter: usize,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let bounds = cfg.bounds();
    let filename = cfg
        .png_out
        .join(PathBuf::from(format!("iteration-{:05}.png", iter)));

    let root = BitMapBackend::new(&filename, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("K-means (k={}, iteration={})", cfg.k, iter),
            ("sans-serif", 50).into_font(),
        )
        .margin(5 as u32)
        .x_label_area_size(30 as u32)
        .y_label_area_size(30 as u32)
        .build_cartesian_2d((bounds.0.x)..(bounds.1.x), (bounds.0.y)..(bounds.1.y))?;
    chart.configure_mesh().draw()?;

    for (centroid, points) in clusters {
        chart.draw_series(PointSeries::of_element(
            points.iter().map(|p| (p.x, p.y)),
            5,
            &Palette99::pick(centroid.color),
            &|c, s: u32, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        chart.draw_series(PointSeries::of_element(
            [(centroid.p.x, centroid.p.y)],
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
    }

    Ok(())
}
