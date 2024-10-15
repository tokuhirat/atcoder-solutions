use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let mut adj = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        adj[a - 1].push(b);
        adj[b - 1].push(a);
    }
    for (i, nodes) in adj.iter().enumerate() {
        print!("{}: {{", i + 1);
        for (j, node) in nodes.iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{}", node);
        }
        println!("}}");
    }
}
