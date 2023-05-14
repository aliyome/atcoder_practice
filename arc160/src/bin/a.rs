use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let f = |l, r| {
        let mut v = a.clone();
        v[l..=r].reverse();
        v
    };

    let mut list = vec![];
    for l in 0..n {
        for r in l..n {
            let v = f(l, r);
            list.push(v);
        }
    }

    list.sort();

    println!("{}", list[k - 1].iter().map(|x| x.to_string()).join(" "));
}
