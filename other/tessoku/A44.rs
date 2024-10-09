use proconio::input;

fn main() {
    input!(
        n: usize,
        q: usize,
    );

    let mut a = (1..=n).collect::<Vec<_>>();
    let mut reverse = false;
    for _ in 0..q {
        input!(num: usize);
        match num {
            1 => {
                input!(x: usize, y: usize);
                if reverse {
                    a[n - x] = y;
                } else {
                    a[x - 1] = y;
                }
            }
            2 => reverse = !reverse,
            3 => {
                input!(x: usize);
                if reverse {
                    println!("{}", a[n - x]);
                } else {
                    println!("{}", a[x - 1]);
                }
            }
            _ => {}
        }
    }
}
