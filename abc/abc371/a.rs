use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        s_ab: char,
        s_ac: char,
        s_bc: char,
    );

    // ABC
    if s_ab == '<' && s_bc == '<' && s_ac == '<' {
        println!("B");
        return;
    }
    // ACB
    if s_ab == '<' && s_bc == '>' && s_ac == '<' {
        println!("C");
        return;
    }
    // BAC
    if s_ab == '>' && s_bc == '<' && s_ac == '<' {
        println!("A");
        return;
    }
    // BCA
    if s_ab == '>' && s_bc == '<' && s_ac == '>' {
        println!("C");
        return;
    }
    // CAB
    if s_ab == '<' && s_bc == '>' && s_ac == '>' {
        println!("A");
        return;
    }
    // CBA
    if s_ab == '>' && s_bc == '>' && s_ac == '>' {
        println!("B");
        return;
    }
}
