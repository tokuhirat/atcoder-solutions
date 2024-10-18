use proconio::{fastout, input};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn root(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            return u;
        }
        self.parent[u] = self.root(self.parent[u]);
        self.parent[u]
    }

    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }

        if self.size[root_u] < self.size[root_v] {
            self.parent[root_u] = root_v;
            self.size[root_v] += self.size[root_u];
        } else {
            self.parent[root_v] = root_u;
            self.size[root_u] += self.size[root_v];
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        query: [(usize, usize, usize); q],
    );

    let parent = (0..n).collect::<Vec<usize>>();
    let size = vec![1; n];

    let mut uf = UnionFind { parent, size };
    for &(t, u, v) in query.iter() {
        let (u, v) = (u - 1, v - 1);
        if t == 1 {
            uf.unite(u, v);
        } else if uf.same(u, v) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
