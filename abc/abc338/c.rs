use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    };
    let mut ans = 0;
    for num_a in 0..=1_000_000 {
        let q_a: Vec<_> = a.iter().map(|e| e * num_a).collect();
        if q.iter().zip(q_a.iter()).any(|(&qi, &q_ai)| qi < q_ai) {
            break;
        }
        let q_b: Vec<_> = q
            .iter()
            .zip(q_a.iter())
            .map(|(&qi, &q_ai)| qi - q_ai)
            .collect();
        let num_b = q_b
            .iter()
            .zip(b.iter())
            .filter(|(_, &bi)| bi != 0)
            .map(|(&q_bi, &bi)| q_bi / bi)
            .min()
            .unwrap();
        ans = ans.max(num_a + num_b);
    }
    println!("{}", ans);
}
