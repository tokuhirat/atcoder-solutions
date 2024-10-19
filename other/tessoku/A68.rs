use proconio::{fastout, input};

#[derive(Clone, Debug, Copy)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

#[derive(Debug)]
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
            if self.graph[pos][i].cap == 0 {
                continue;
            }
            if self.used[self.graph[pos][i].to] {
                continue;
            }
            let f = self.dfs(
                self.graph[pos][i].to,
                goal,
                flow.min(self.graph[pos][i].cap),
            );
            if f > 0 {
                self.graph[pos][i].cap -= f;
                let to = self.graph[pos][i].to;
                let rev = self.graph[pos][i].rev;
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
                self.used[i] = false
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
        m: usize,
        abc: [(usize, usize, usize); m],
    );

    let mut mf = MaximumFlow::new(n);
    for &(a, b, c) in abc.iter() {
        mf.add_edge(a - 1, b - 1, c);
    }

    println!("{}", mf.max_flow(0, n - 1));
}
