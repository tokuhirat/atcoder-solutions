use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abcd: [usize; 4],
    };
    abcd.sort();
    if abcd[0] == abcd[1] && abcd[2] == abcd[3] && abcd[1] != abcd[2] {
        println!("Yes");
        return;
    }
    if abcd[0] != abcd[1] && abcd[1] == abcd[2] && abcd[2] == abcd[3] {
        println!("Yes");
        return;
    }
    if abcd[0] == abcd[1] && abcd[1] == abcd[2] && abcd[2] != abcd[3] {
        println!("Yes");
        return;
    }
    println!("No");
}
