
pub trait Graph<V, E>: From<String>
    where V: PartialEq + Clone,
          E: PartialEq + Clone
{
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn vertices(&self) -> Vec<V>;
    #[allow(non_snake_case)]
    fn V(&self) -> Vec<V> {
        self.vertices()
    }
    fn neighbourhood(&self, u: &V) -> &Vec<E>;
    #[allow(non_snake_case)]
    fn N(&self, u: &V) -> &Vec<E> {
        self.neighbourhood(u)
    }
    fn set_neighbourhood(&mut self, u: V, neigh: Vec<E>);
    fn add_edge(&mut self, u: V, e: E);
    fn add_edge_from_str(&mut self, edge: &str) -> Option<()>;
}
