use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut stack = vec![];
    let mut ans = vec![0; n];
    for i in 0..n {
        if i > 0 {
            stack.push((i - 1, a[i - 1]));
            while !stack.is_empty() {
                if stack.last().unwrap().1 <= a[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        if !stack.is_empty() {
            ans[i] = stack.last().unwrap().0 as i32 + 1;
        } else {
            ans[i] = -1;
        }
    }
    for (i, &ans_i) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans_i);
    }
    println!("");
}
