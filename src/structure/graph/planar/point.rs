use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Neg;
use std::str::FromStr;
use num::Complex;
use crate::structure::graph::planar::pre_planar_edge::PrePlanarEdge;
use crate::structure::weight::Weight;

#[derive(PartialEq, Clone)]
pub struct Point {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ws = s.split(' ');
        Ok(Point {
            id: ws.next().ok_or("Could not find the id")?.parse().or(Err("Could not parse the id"))?,
            x: ws.next().ok_or("Could not find the x coordinate")?.parse().or(Err("Could not parse the x coordinate"))?,
            y: ws.next().ok_or("Could not find the y coordinate")?.parse().or(Err("Could not parse the y coordinate"))?,
        })
    }
}

pub fn compare_edges_clockwise<'a, W: Weight>(center: &'a Point, points: &'a Vec<Point>, edges: &'a Vec<PrePlanarEdge<W>>) -> impl FnMut(&usize, &usize) -> Ordering + 'a {
    |&a, &b| {
        let fa = angle_from_center(center, &points[edges[a].to]);
        let fb = angle_from_center(center, &points[edges[b].to]);
        if fa < fb { Less }
        else if fa > fb { Greater }
        else { Equal }
    }
}

fn angle_from_center(center: &Point, other: &Point) -> f64 {
    return (Complex::new(other.x, other.y) - Complex::new(center.x, center.y)).to_polar().1.neg();
}
