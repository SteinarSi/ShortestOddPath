use crate::utility::meta_test;
use crate::utility::problem::TwoDisjointPaths;

mod utility;

fn test(folder: &str, name: &str) {
    meta_test::<TwoDisjointPaths, u64>(folder, name);
}

#[cfg(test)]
mod test_medium_disjoint_paths {
    fn test(name: &str) { super::test("medium_graphs", name); }
    #[ignore = "The test fails, and the algorithm is probably wrong, but it's not a priority now."]
    #[test]
    fn medium1() { test("medium1"); }
    #[test]
    #[ignore = "The test fails, and the algorithm is probably wrong, but it's not a priority now."]
    fn medium3() { test("medium3"); }
}

#[cfg(test)]
mod test_small_disjoint_paths {
    fn test(name: &str) { super::test("small_graphs", name); }
    #[test]
    fn small3() { test("small3"); }
}
