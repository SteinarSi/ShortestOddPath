use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Neg;
use std::str::FromStr;
use num::Complex;
use crate::structure::graph::planar_edge::PrePlanarEdge;
use crate::structure::weight::Weight;

#[derive(PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x,
            y
        }
    }
}

pub fn compare_edges_clockwise<'a, W: Weight>(center: &'a Point, points: &'a Vec<Point>) -> impl FnMut(&PrePlanarEdge<W>, &PrePlanarEdge<W>) -> Ordering + 'a {
    |a, b| {
        let fa = angle_from_center(center, &points[a.to]);
        let fb = angle_from_center(center, &points[b.to]);
        if fa < fb { Less }
        else if fa > fb { Greater }
        else { Equal }
    }
}

fn angle_from_center(center: &Point, other: &Point) -> f64 {
    return (Complex::new(other.x, other.y) - Complex::new(center.x, center.y)).to_polar().1.neg();
}

#[cfg(test)]
mod test_points {
    use crate::structure::graph::edge::map_to;
    use super::*;
    fn new_edge(u:usize, v: usize) -> PrePlanarEdge<u64> {
        PrePlanarEdge {
            from: u,
            to: v,
            weight: 0,
            left: None,
            right: None,
        }
    }
    #[test]
    fn test_sorting() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, -1.0),
            Point::new(2.0, 7.0),
            Point::new(12.0, 6.0),
            Point::new(5.0, 3.0),
            Point::new(7.0, 10.0),
            Point::new(10.0, 3.0),
            Point::new(0.0, 4.0),
            Point::new(4.0, -2.0),
        ];
        let mut edges = vec![
            new_edge(4, 3),
            new_edge(4, 1),
            new_edge(4, 5),
            new_edge(4, 2),
            new_edge(4, 0),
            new_edge(4, 8),
            new_edge(4, 6),
            new_edge(4, 7),
        ];
        edges.sort_by(compare_edges_clockwise(&points[4], &points));
        assert_eq!(map_to(&edges), vec![7, 2, 5, 3, 6, 1, 8, 0]);
    }
}