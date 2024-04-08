use std::str::Split;

pub trait Graph<'a, V, E>
    where V: PartialEq + Clone + 'a,
          E: PartialEq + Clone,
{
    fn n(&self) -> usize;
    fn m(&self) -> usize;
    fn vertices(&'a self) -> impl Iterator<Item = V>;
    #[allow(non_snake_case)]
    fn V(&'a self) -> impl Iterator<Item = V> {
        self.vertices()
    }
    fn add_edge(&mut self, u: V, e: E);
    fn parse_vertex(&self, rs: &mut Split<char>) -> Result<usize, String> {
        let next = rs.next().ok_or("Expected an unsigned integer here, but found nothing!")?;
        let u = next.parse().or(Err(format!("Could not read '{}' as an unsigned integer!", next)))?;
        if u >= self.n() {
            Err(format!("The parsed vertex {} is too large for a graph of size {}", u, self.n()))
        }
        else {
            Ok(u)
        }
    }
}
