extern crate polyclip;

use polyclip::*;

fn main() {

    // triangle structure
    let subject_polygon = Polygon::new(vec![
        Point2D { x: 5.0, y: 5.0 },
        Point2D { x: 10.0, y: 10.0 },
        Point2D { x: 10.0, y: 5.0 },
    ]);

    // same as subject_polygon but offset by 2.0 in x
    let clip_polygon = Polygon::new(vec![
        Point2D { x: 7.0, y: 5.0 },
        Point2D { x: 12.0, y: 10.0 },
        Point2D { x: 12.0, y: 5.0 },
    ]);

    println!("subject polygon: {:?}", subject_polygon.subtract(&clip_polygon));
}
