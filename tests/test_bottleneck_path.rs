use crate::utility::problem::ShortestBottleneckPath;
use crate::utility::meta_test;

mod utility;

fn test(folder: &str, name: &str) {
    meta_test::<ShortestBottleneckPath, u64>(folder, name);
}

#[test]
fn medium1() { test("medium_graphs", "medium1"); }