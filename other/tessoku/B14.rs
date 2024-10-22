use proconio::{fastout, input};

fn enumarate(a: &[&usize]) -> Vec<usize> {
    let mut ret = vec![];
    for bit in 0..1 << a.len() {
        let mut v = 0;
        for (i, &ai) in a.iter().enumerate() {
            if (bit >> i) & 1 == 1 {
                v += ai;
            }
        }
        ret.push(v);
    }
    ret.sort_unstable();
    ret
}

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        a: [usize; n],
    );

    let a1 = a.iter().take(n / 2).collect::<Vec<_>>();
    let a2 = a.iter().take(n).skip(n / 2).collect::<Vec<_>>();

    let p1 = enumarate(&a1);
    let p2 = enumarate(&a2);

    for &x in p1.iter() {
        let pos = p2.binary_search(&(k - x));
        if pos.is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
