use proconio::{fastout, input};
use std::collections::VecDeque;

fn next_state(n: usize, current: usize, operation: (usize, usize, usize)) -> usize {
    let mut state = vec![0; n];
    for (i, si) in state.iter_mut().enumerate() {
        if (current >> i) & 1 == 1 {
            *si = 1;
        }
    }
    let (x, y, z) = operation;
    let (x, y, z) = (x - 1, y - 1, z - 1);
    state[x] = 1 - state[x];
    state[y] = 1 - state[y];
    state[z] = 1 - state[z];

    let mut next = 0;
    for (i, &si) in state.iter().enumerate() {
        if si == 1 {
            next += 1 << i;
        }
    }
    next
}

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(usize, usize, usize);m],
    );
    let mut graph = vec![vec![]; 1 << n];
    for (i, gi) in graph.iter_mut().enumerate() {
        for &operation in xyz.iter() {
            let state = next_state(n, i, operation);
            gi.push(state);
        }
    }

    let mut distance = vec![-1; 1 << n];
    let mut start = 0;
    for (i, &ai) in a.iter().enumerate() {
        if ai == 1 {
            start += 1 << i;
        }
    }
    let goal = (1 << n) - 1;
    let mut queue = VecDeque::from([start]);
    distance[start] = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for &next_node in graph[current].iter() {
            if distance[next_node] == -1 {
                distance[next_node] = distance[current] + 1;
                queue.push_back(next_node);
            }
        }
    }
    println!("{}", distance[goal]);
}
