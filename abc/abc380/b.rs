use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    let mut ans = vec![];
    let mut l = 0;
    for &si in s.iter().skip(1) {
        if si == '|' {
            ans.push(l);
            l = 0;
        } else {
            l += 1;
        }
    }
    for (i, &ai) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!("");
}
