use proconio::{fastout, input};

fn f(a: &[usize], b: &[usize]) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let mut a_even = Vec::with_capacity(a.len());
    let mut a_odd = Vec::with_capacity(a.len());
    let mut b_even = Vec::with_capacity(b.len());
    let mut b_odd = Vec::with_capacity(b.len());

    for &ai in a {
        if ai % 2 == 0 {
            a_even.push(ai);
        } else {
            a_odd.push(ai);
        }
    }
    for &bi in b {
        if bi % 2 == 0 {
            b_even.push(bi);
        } else {
            b_odd.push(bi);
        }
    }
    let ee = f(
        &a_even.iter().map(|&e| e / 2).collect::<Vec<usize>>(),
        &b_even.iter().map(|&e| e / 2).collect::<Vec<usize>>(),
    );
    let eo =
        b_odd.len() * a_even.iter().sum::<usize>() + a_even.len() * b_odd.iter().sum::<usize>();
    let oe =
        a_odd.len() * b_even.iter().sum::<usize>() + b_even.len() * a_odd.iter().sum::<usize>();
    let oo = f(
        &a_odd.iter().map(|e| e / 2).collect::<Vec<usize>>(),
        &b_odd.iter().map(|e| e / 2 + 1).collect::<Vec<usize>>(),
    );
    ee + eo + oe + oo
}

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );
    let mut ans: usize = 0;
    for &ai in &a {
        let mut ai = ai;
        while ai % 2 == 0 {
            ai /= 2;
        }
        ans += ai;
    }
    ans += f(&a, &a);
    ans /= 2;
    println!("{}", ans);
}
