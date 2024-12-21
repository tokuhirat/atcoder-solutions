use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut abc: [usize; 3],
    );
    abc.sort();
    if abc[0] + abc[1] == abc[2] {
        println!("Yes");
        return;
    }
    if abc[0] == abc[1] && abc[1] == abc[2] {
        println!("Yes");
        return;
    }
    println!("No");
}
