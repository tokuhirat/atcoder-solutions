use proconio::{fastout, input, marker::Chars};

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

struct MaximumFlow {
    graph: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl MaximumFlow {
    fn new(n: usize) -> MaximumFlow {
        MaximumFlow {
            graph: vec![vec![]; n],
            used: vec![false; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
        let size_from = self.graph[from].len();
        let size_to = self.graph[to].len();
        self.graph[from].push(Edge {
            to,
            cap,
            rev: size_to,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev: size_from,
        });
    }

    fn dfs(&mut self, pos: usize, goal: usize, flow: usize) -> usize {
        if pos == goal {
            return flow;
        }
        self.used[pos] = true;
        for i in 0..self.graph[pos].len() {
            let to = self.graph[pos][i].to;
            let cap = self.graph[pos][i].cap;
            let rev = self.graph[pos][i].rev;
            if cap == 0 {
                continue;
            }
            if self.used[to] {
                continue;
            }
            let f = self.dfs(to, goal, flow.min(cap));
            if f > 0 {
                self.graph[pos][i].cap -= f;
                self.graph[to][rev].cap += f;
                return f;
            }
        }
        0
    }

    fn max_flow(&mut self, start: usize, goal: usize) -> usize {
        let mut total_flow = 0;
        loop {
            for i in 0..self.used.len() {
                self.used[i] = false;
            }
            let f = self.dfs(start, goal, usize::MAX);
            if f == 0 {
                break;
            }
            total_flow += f;
        }
        total_flow
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
        c: [Chars; n],
    );

    let mut mf = MaximumFlow::new(2 * n + 2);
    for (i, ci) in c.iter().enumerate() {
        for (j, cij) in ci.iter().enumerate() {
            if cij == &'#' {
                mf.add_edge(i, n + j, 1);
            }
        }
    }
    for i in 0..n {
        mf.add_edge(2 * n, i, 1);
        mf.add_edge(n + i, 2 * n + 1, 1);
    }
    let ans = mf.max_flow(2 * n, 2 * n + 1);
    println!("{}", ans);
}
