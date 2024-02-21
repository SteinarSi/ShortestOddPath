mod utility;

use shortest_odd_path::algorithm::odd_walk::BasicOddWalk;
use crate::utility::test_s_t_trip;


#[test]
fn it_tests() {
    assert_eq!(2, 1+1);
}

fn test_walk(folder: &str, file: &str) {
    test_s_t_trip::<BasicOddWalk>(folder, file)
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