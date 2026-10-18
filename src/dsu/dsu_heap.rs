/// Disjoint-set data structure.
pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
    cnt: usize,
}

impl DSU {
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            cnt: n,
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<usize>>(),
        }
    }
    #[inline]
    pub fn union(&mut self, x: usize, y: usize) {
        super::dsu_union(&mut self.parent, &mut self.rank, &mut self.cnt, x, y);
    }
    #[inline]
    pub fn find(&mut self, x: usize) -> usize {
        super::dsu_find(&mut self.parent, x)
    }
    #[inline]
    pub fn cnt(&self) -> usize {
        self.cnt
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dsu_test() {}
}
