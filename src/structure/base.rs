use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::ptr::write;

pub struct Base {
    basis: Vec<usize>,
}

impl Base {
    pub fn new(n: usize) -> Self {
        Base {
            basis: (0..n).collect(),
        }
    }

    pub fn get_base(&self, u: usize) -> &usize {
        if u != self.basis[u] {
            let base_base = self[self.basis[u]];
            unsafe {
                // Path compression.
                // The vector is invisible to the outside world, therefore it should be safe to mutate it here.
                write((self.basis.as_ptr() as *mut usize).add(u), base_base);
            }
        }
        return &self.basis[u];
    }

    pub fn set_base(&mut self, u: usize, new_base: usize) {
        self[u] = new_base;
    }

    pub fn same_base(&self, u: usize, v: usize) -> bool {
        self[u] == self[v]
    }
}

impl Index<usize> for Base {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_base(index)
    }
}

impl IndexMut<usize> for Base {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let base = self[index];
        &mut self.basis[base]
    }
}

impl Debug for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (0..self.basis.len()).map(|u| self[u]).collect::<Vec<usize>>())
    }
}

#[cfg(test)]
mod test_base {
    use super::*;

    #[test]
    fn test_base() {
        let mut base = Base::new(10);

        assert!( ! base.same_base(0, 1));
        assert!( ! base.same_base(2, 3));

        base[1] = 0;
        assert!(base.same_base(0, 1));
        assert!(base.same_base(1, 0));
        assert!( ! base.same_base(2, 3));

        base[3] = 4;
        base[5] = 4;
        base[5] = 6;
        base[7] = 8;
        base[9] = 5;
        base[5] = 8;

        assert_eq!(base[5], 8);
        assert_eq!(base[1], 0);
        assert!(base.same_base(7, 9));
        assert!( ! base.same_base(1, 4));
    }
}