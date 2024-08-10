use nannou::prelude::*;

pub fn circle_points(radius: f32, num_points: usize) -> Vec<Point2> {
    (0..num_points)
        .map(|i| {
            let angle = map_range(i, 0, num_points, 0.0, 2.0 * PI);
            pt2(radius * angle.cos(), radius * angle.sin())
        })
        .collect()
}

pub fn grid_points(rows: usize, cols: usize, spacing: f32) -> Vec<Point2> {
    let mut points = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let x = j as f32 * spacing - (cols as f32 * spacing) / 2.0;
            let y = i as f32 * spacing - (rows as f32 * spacing) / 2.0;
            points.push(pt2(x, y));
        }
    }
    points
}