use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut m: usize,
    );
    let mut ans = vec![];
    let mut s = 0;
    let mut a = 11;
    while s < m && a > 0 {
        a -= 1;
        while s + (3_i32.pow(a) as usize) <= m {
            ans.push(a);
            s += 3_i32.pow(a) as usize;
        }
    }
    println!("{}", ans.len());
    for (i, &ai) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!("");
}
