use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );
    let mut ans = vec![0; n];
    let mut queue = vec![];
    for i in (0..n - 1).rev() {
        while !queue.is_empty() && queue.last().unwrap() < &h[i + 1] {
            queue.pop();
        }
        queue.push(h[i + 1]);
        ans[i] = queue.len();
    }
    for (i, &ai) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!("");
}
