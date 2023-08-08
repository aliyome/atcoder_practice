use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut ans = 0;
    for i in 1..=100000 {
        for &(l, r) in &lr {
            if l <= i && i <= r {
                ans += 1
            }
        }
    }

    println!("{}", ans);
}
