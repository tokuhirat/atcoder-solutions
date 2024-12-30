use std::io::{stdin, stdout, BufReader, Write};

use itertools::Itertools;
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize
    };
    let m = n.next_power_of_two().trailing_zeros();
    println!("{}", m);
    for i in 0..m {
        let mut v = vec![];
        for bi in 0..n {
            if bi & (1 << i) != 0 {
                v.push(bi + 1);
            }
        }
        println!("{} {}", v.len(), v.iter().join(" "));
    }
    stdout().flush().unwrap();
    input! {
        from &mut source,
        s: Chars
    };
    let mut ans = 0;
    for (i, &si) in s.iter().enumerate() {
        if si == '1' {
            ans += 1 << i;
        }
    }
    println!("{}", ans + 1);
    stdout().flush().unwrap();
}
