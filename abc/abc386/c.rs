use proconio::{input, marker::Chars};

fn main() {
    input! {
        _k:usize,
        s: Chars,
        t: Chars,
    };
    match s.len().abs_diff(t.len()) {
        0 => {
            let mut c = 0;
            for i in 0..s.len() {
                if s[i] != t[i] {
                    c += 1;
                }
                if c > 1 {
                    println!("No");
                    return;
                }
            }
            println!("Yes");
        }
        1 => {
            if s.len() > t.len() {
                let mut c = 0;
                let mut i = 0;
                let mut j = 0;
                while i < s.len() && j < t.len() {
                    if s[i] != t[j] {
                        c += 1;
                        i += 2;
                        j += 1;
                    } else {
                        i += 1;
                        j += 1;
                    }
                    if c > 1 {
                        println!("No");
                        return;
                    }
                }
            } else {
                let mut c = 0;
                let mut i = 0;
                let mut j = 0;
                while i < s.len() && j < t.len() {
                    if s[i] != t[j] {
                        c += 1;
                        i += 1;
                        j += 2;
                    } else {
                        i += 1;
                        j += 1;
                    }
                    if c > 1 {
                        println!("No");
                        return;
                    }
                }
            }
            println!("Yes");
        }
        _ => println!("No"),
    }
}
