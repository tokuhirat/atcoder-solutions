use proconio::{fastout, input};

fn dfs(ab: &Vec<(usize, usize)>, used: &mut Vec<bool>, table: Vec<Vec<bool>>) -> bool {
    let (h, w) = (table.len(), table[0].len());

    let (mut x, mut y) = (-1, -1);
    'outer: for (i, ti) in table.iter().enumerate() {
        for (j, &tij) in ti.iter().enumerate() {
            if !tij {
                x = i as i32;
                y = j as i32;
                break 'outer;
            }
        }
    }
    if x == -1 {
        return true;
    }
    let (x, y) = (x as usize, y as usize);

    for (i, &(a, b)) in ab.iter().enumerate() {
        if used[i] {
            continue;
        }

        let (mut a, mut b) = (a, b);
        't: for _ in 0..2 {
            (b, a) = (a, b);

            if x + a > h || y + b > w {
                continue;
            }

            for ti in table.iter().skip(x).take(a) {
                for &tij in ti.iter().skip(y).take(b) {
                    if tij {
                        continue 't;
                    }
                }
            }

            let mut nxt_table = table.clone();
            for ti in nxt_table.iter_mut().skip(x).take(a) {
                for tij in ti.iter_mut().skip(y).take(b) {
                    *tij = true;
                }
            }
            used[i] = true;
            if dfs(ab, used, nxt_table) {
                return true;
            }
            used[i] = false;
        }
    }
    false
}

#[fastout]
fn main() {
    input!(
        (n, h, w): (usize, usize, usize),
        ab: [(usize, usize); n],
    );
    let mut used = vec![false; n];
    let ans = dfs(&ab, &mut used, vec![vec![false; w]; h]);

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
