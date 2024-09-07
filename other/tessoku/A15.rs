use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(
        n: usize,
        a: [i32; n]
    );

    let mut num_to_idx = HashMap::new();

    for (i, &ai) in a.iter().enumerate() {
        num_to_idx.entry(ai).or_insert(Vec::new()).push(i);
    }

    let mut num_in_a = num_to_idx.keys().collect::<Vec<&i32>>();
    num_in_a.sort();

    let mut b = vec![0; n];
    for (i, &ai) in num_in_a.iter().enumerate() {
        for &idx in num_to_idx.get(ai).unwrap() {
            b[idx] = i + 1
        }
    }

    let ans = b
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
