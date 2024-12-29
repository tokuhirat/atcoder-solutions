use ac_library::{Max, Segtree};
use proconio::{fastout, input};

const SIZE: usize = 500_001;
#[fastout]
fn main() {
    input! {
        (n, d): (usize, usize),
        a: [usize; n],
    };
    let mut sg = Segtree::<Max<usize>>::new(SIZE);
    for &ai in &a {
        let l = ai.saturating_sub(d);
        let r = (SIZE - 1).min(ai + d);
        let max = sg.prod(l..=r);
        sg.set(ai, max + 1);
    }
    println!("{}", sg.all_prod());
}
