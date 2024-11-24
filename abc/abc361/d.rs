use proconio::{fastout, input, marker::Chars};
use std::collections::{HashMap, VecDeque};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut s: Chars,
        mut t: Chars,
    );

    let s_b: usize = s.iter().map(|&x| if x == 'B' { 1 } else { 0 }).sum();
    let t_b: usize = t.iter().map(|&x| if x == 'B' { 1 } else { 0 }).sum();
    if s_b != t_b {
        println!("-1");
        return;
    }

    s.append(&mut vec!['.', '.']);
    t.append(&mut vec!['.', '.']);
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    queue.push_back(s.clone());
    dist.insert(s, 0);
    while let Some(cur) = queue.pop_front() {
        if cur == t {
            println!("{}", dist[&cur]);
            return;
        }

        let idx = cur.iter().position(|&x| x == '.').unwrap();
        for i in 0..=n {
            if cur[i] == '.' || cur[i + 1] == '.' {
                continue;
            }
            let mut nxt = cur.clone();
            (nxt[i], nxt[i + 1], nxt[idx], nxt[idx + 1]) =
                (nxt[idx], nxt[idx + 1], nxt[i], nxt[i + 1]);
            if !dist.contains_key(&nxt) {
                queue.push_back(nxt.clone());
                dist.insert(nxt, dist[&cur] + 1);
            }
        }
    }
    println!("-1");
}
