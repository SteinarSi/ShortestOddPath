mod utility;

use crate::utility::problem::ShortestOddWalk;
use crate::utility::meta_test;

fn test_walk(folder: &str, file: &str) {
    meta_test::<ShortestOddWalk>(folder, file)
}

mod small_walks {
    use crate::test_walk;

    fn test(name: &str) { test_walk("small_graphs", name); }

    #[test]
    fn small1() { test("small1"); }
    #[test]
    fn small2() { test("small2"); }
    #[test]
    fn small3() { test("small3"); }
    #[test]
    fn small4() { test("small4"); }
}