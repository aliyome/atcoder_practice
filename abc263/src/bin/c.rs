use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let list = (1..=m).combinations(n).collect_vec();

    for v in list {
        println!("{}", v.iter().join(" "));
    }
}
