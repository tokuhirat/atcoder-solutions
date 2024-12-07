use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

struct TrieNode {
    to: HashMap<char, TrieNode>,
    cnt: usize,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            to: HashMap::new(),
            cnt: 0,
        }
    }

    fn add(&mut self, s: &[char]) {
        let mut node = self;
        for &si in s {
            node = node.to.entry(si).or_insert_with(TrieNode::new);
            node.cnt += 1;
        }
    }

    fn dfs(&self) -> usize {
        let mut ret = self.cnt * (self.cnt.max(1) - 1) / 2;
        for (_, node) in self.to.iter() {
            ret += node.dfs();
        }
        ret
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
        s: [Chars; n],
    );
    let mut trie = TrieNode::new();

    for si in &s {
        trie.add(si);
    }
    let ans = trie.dfs();
    println!("{}", ans);
}
