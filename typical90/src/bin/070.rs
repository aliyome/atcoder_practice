use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let xa = xy.iter().map(|&(x, _)| x).sorted().collect_vec()[n / 2];
    let ya = xy.iter().map(|&(_, y)| y).sorted().collect_vec()[n / 2];

    let mut ans = 0;
    for &(x, y) in &xy {
        ans += (x - xa).abs() + (y - ya).abs();
    }

    println!("{}", ans);
}
