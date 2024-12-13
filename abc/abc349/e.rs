use proconio::{fastout, input};

fn win(h: &[u8]) -> bool {
    for i in 0..3 {
        if h.contains(&(3 * i)) && h.contains(&(3 * i + 1)) && h.contains(&(3 * i + 2)) {
            return true;
        }
    }
    for i in 0..3 {
        if h.contains(&(i)) && h.contains(&(3 + i)) && h.contains(&(6 + i)) {
            return true;
        }
    }
    if h.contains(&0) && h.contains(&4) && h.contains(&8) {
        return true;
    }
    if h.contains(&2) && h.contains(&4) && h.contains(&6) {
        return true;
    }
    false
}

fn check(a: &[Vec<i64>], h: &[u8], sum: i64) -> char {
    let hi: Vec<_> = h
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != h.len() % 2)
        .map(|(_, &j)| j)
        .collect();
    if win(&hi) {
        if h.len() % 2 == 0 {
            return 'a';
        } else {
            return 't';
        }
    }
    if h.len() == 9 {
        let mut t = 0;
        for i in 0..=4 {
            t += a[(h[2 * i] / 3) as usize][(h[2 * i] % 3) as usize];
        }
        if t > sum - t {
            't'
        } else {
            'a'
        }
    } else {
        '0'
    }
}
fn solv(a: &[Vec<i64>], h: &mut Vec<u8>, sum: i64) -> char {
    match check(a, h, sum) {
        't' => return 't',
        'a' => return 'a',
        _ => (),
    }
    let mut r = vec![];
    for i in 0..9 {
        if h.contains(&i) {
            continue;
        }
        h.push(i);
        r.push(solv(a, h, sum));
        h.pop();
    }
    if h.len() % 2 == 0 {
        if r.contains(&'t') {
            't'
        } else {
            'a'
        }
    } else if r.contains(&'a') {
        'a'
    } else {
        't'
    }
}

#[fastout]
fn main() {
    input!(
        a: [[i64; 3]; 3],
    );
    let mut h = vec![];
    let mut sum = 0;
    for ai in &a {
        for aij in ai {
            sum += aij;
        }
    }
    let ans = solv(&a, &mut h, sum);
    if ans == 't' {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
