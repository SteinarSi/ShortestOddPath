#![allow(dead_code)]

use crate::algorithm::algorithm::PathResult::*;
use crate::algorithm::odd_path::DerigsAlgorithm;
use crate::utility::testing::test_s_t_trip;

pub mod structure;
pub mod utility;
pub mod algorithm;

fn main() {
    test_s_t_trip::<DerigsAlgorithm>("small_graphs", "small2");
}