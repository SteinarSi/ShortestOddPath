use crate::structure::weight::Weight;

#[derive(PartialEq, Clone)]
pub struct Line<W: Weight> {
    pub from: usize,
    pub to: usize,
    pub weight: W,
    pub left: usize,
    pub right: usize,
}

impl <W: Weight> Line<W> {
    pub fn reverse(&self) -> Self {
        Line {
            from: self.to,
            to: self.from,
            weight: self.weight,
            left: self.right,
            right: self.left,
        }
    }
}