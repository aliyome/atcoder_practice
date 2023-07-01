use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    let mut list = vec![];
    for i in 0..n {
        let rate = ab[i].0 as f64 / (ab[i].0 as f64 + ab[i].1 as f64);
        list.push((rate, i + 1));
    }

    list.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(a.1.cmp(&b.1)));

    for i in 0..n {
        print!("{} ", list[i].1);
    }
}
