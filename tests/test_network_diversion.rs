use crate::utility::meta_test;
use crate::utility::problem::NetworkDiversion;

mod utility;

fn test_diversion(name: &str) {
    meta_test::<NetworkDiversion, f64>("planar_graphs", name);
}

#[test]
fn planar1() { test_diversion("small_planar1"); }
