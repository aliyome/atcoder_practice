use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    };

    let mut list = ab.into_iter().enumerate().collect_vec();
    list.sort_by(|(i, a), (j, b)| (b.0 * (a.0 + a.1)).cmp(&(a.0 * (b.0 + b.1))).then(i.cmp(j)));

    for (i, _) in list {
        print!("{} ", i + 1);
    }
}
