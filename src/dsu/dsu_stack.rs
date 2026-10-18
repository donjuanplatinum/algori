/// Disjoint-set data structure on the stack.
pub struct DSU<const N: usize> {
    parent: [usize; N],
    rank: [usize; N],
    cnt: usize,
}
impl<const N: usize> DSU<N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            cnt: N,
            rank: [0; N],
            parent: core::array::from_fn(|i| i),
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
