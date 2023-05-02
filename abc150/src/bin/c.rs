use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    };

    let perm = (1..=n).permutations(n).collect_vec();

    let a = perm.iter().position(|x| x == &p).unwrap();
    let b = perm.iter().position(|x| x == &q).unwrap();

    println!("{}", (a as isize - b as isize).abs());
}
