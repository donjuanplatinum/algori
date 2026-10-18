#[cfg(feature = "alloc")]
mod dsu_heap;

#[cfg(feature = "alloc")]
pub use dsu_heap::DSU;

#[cfg(not(feature = "alloc"))]
mod dsu_stack;

#[cfg(not(feature = "alloc"))]
pub use dsu_stack::DSU;

/// Union two root.
fn dsu_union(parent: &mut [usize], rank: &mut [usize], cnt: &mut usize, x: usize, y: usize) {
    let mut x_root = dsu_find(parent, x);
    let mut y_root = dsu_find(parent, y);

    if x_root == y_root {
        return;
    }

    if rank[x_root] < rank[y_root] {
        core::mem::swap(&mut x_root, &mut y_root);
    }

    parent[y_root] = x_root;

    if rank[x_root] == rank[y_root] {
        rank[x_root] += 1;
    }

    *cnt -= 1;
}
fn dsu_find(parent: &mut [usize], x: usize) -> usize {
    let mut cur = x;

    while cur != parent[cur] {
        // Path Halving.
        // cur.parent -> cur.parent.parent
        let p = parent[cur];
        parent[cur] = parent[p];
        cur = parent[cur];
    }

    cur
}
