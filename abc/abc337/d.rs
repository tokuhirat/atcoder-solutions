use proconio::{fastout, input, marker::Chars};

fn f(s: &[char], k: usize) -> usize {
    let mut right = 0;
    let mut ret = usize::MAX;
    let mut cnt = 0;
    for (left, &sl) in s.iter().enumerate() {
        if sl == 'x' {
            continue;
        }
        if left > 0 && s[left - 1] == '.' {
            cnt -= 1;
        }

        if right < left {
            right = left;
            cnt = 0;
        }
        while right < s.len() && s[right] != 'x' {
            if s[right] == '.' {
                cnt += 1;
            }
            right += 1;
            if right - left >= k {
                ret = ret.min(cnt);
                break;
            }
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        (h, w, k): (usize, usize, usize),
        s: [Chars; h],
    };

    let mut ans = usize::MAX;
    for si in &s {
        ans = ans.min(f(si, k));
    }
    for j in 0..w {
        let sj: Vec<_> = (0..h).map(|i| s[i][j]).collect();
        ans = ans.min(f(&sj, k));
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
