use crate::utility::problem::ShortestOddPath;
use shortest_odd_path::utility::misc::debug;
use crate::utility::meta_test;
mod utility;

fn test_path(folder: &str, name: &str) {
    debug(format!("{}/{}", folder, name));
    meta_test::<ShortestOddPath, u64>(folder, name)
}
mod small_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("small_graphs", name); }

    #[test]
    fn small1() { test("small1"); }
    #[test]
    fn small2() { test("small2"); }
    #[test]
    fn small3() { test("small3"); }
    #[test]
    fn small4() { test("small4"); }
    #[test]
    fn small5() { test("small5"); }
    #[test]
    fn small6() { test("small6"); }
    #[test]
    fn small7() { test("small7"); }
}

mod medium_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("medium_graphs", name); }

    #[test]
    fn medium1() { test("medium1"); }
    #[test]
    fn medium2() { test("medium2"); }
    #[test]
    fn medium3() { test("medium3"); }
    #[test]
    fn medium4() { test("medium4"); }
    #[test]
    fn medium5() { test("medium5"); }
    #[test]
    fn medium6() { test("medium6"); }
}

mod large_paths {
    use crate::test_path;

    fn test(name: &str) { test_path("large_graphs", name); }

    #[test]
    fn large1() { test("large1"); }
    #[test]
    fn large2() { test("large2"); }
}

mod special_paths {
    fn test(name: &str) { crate::test_path("special_graphs", name); }

    #[test]
    fn gamma4() { test("gamma4"); }
}
