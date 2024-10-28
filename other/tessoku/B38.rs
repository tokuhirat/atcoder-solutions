use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars
    );

    let mut height = vec![0; n];
    for (i, &si) in s.iter().enumerate() {
        if i == 0 && si == 'A' {
            height[i] = 1;
        }
        if i == n - 2 && si == 'B' {
            height[i + 1] = 1;
        }
        if i > 0 && si == 'A' && s[i - 1] == 'B' {
            height[i] = 1;
        }
    }

    for i in 1..n {
        if s[i - 1] == 'A' {
            height[i] = height[i - 1] + 1;
        }
    }

    for i in (0..n - 1).rev() {
        if s[i] == 'B' {
            height[i] = height[i].max(height[i + 1] + 1);
        }
    }
    println!("{}", height.iter().sum::<usize>())
}
