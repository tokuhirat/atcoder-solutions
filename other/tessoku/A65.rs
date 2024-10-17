use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n - 1],
    );

    let mut tree = vec![vec![]; n + 1];
    for (i, &ai) in a.iter().enumerate() {
        tree[ai].push(i + 2);
    }

    let mut ans = vec![-1; n + 1];
    dfs(1, &tree, &mut ans);

    for (i, &ai) in ans.iter().skip(1).enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", ai);
    }
    println!("");
}

fn dfs(num: usize, tree: &[Vec<usize>], ans: &mut [i32]) -> usize {
    if tree[num].is_empty() {
        ans[num] = 0;
        return 1;
    }
    let mut ret = 0;
    for &next_node in tree[num].iter() {
        ret += dfs(next_node, tree, ans);
    }
    ans[num] = ret as i32;
    ret + 1
}
