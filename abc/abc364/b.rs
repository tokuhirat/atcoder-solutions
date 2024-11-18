use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input!(
        (h, w): (usize, usize),
        (mut si, mut sj): (Usize1, Usize1),
        c: [Chars; h],
        x: Chars
    );

    for &xi in &x {
        match xi {
            'L' if sj > 0 && c[si][sj - 1] == '.' => {
                sj -= 1;
            }
            'R' if sj < w - 1 && c[si][sj + 1] == '.' => {
                sj += 1;
            }
            'U' if si > 0 && c[si - 1][sj] == '.' => {
                si -= 1;
            }
            'D' if si < h - 1 && c[si + 1][sj] == '.' => {
                si += 1;
            }
            _ => (),
        }
    }
    println!("{} {}", si + 1, sj + 1);
}
