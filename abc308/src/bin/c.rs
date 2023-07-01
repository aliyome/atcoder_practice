use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    };

    let mut list = vec![];
    for i in 0..n {
        list.push((ab[i].0, ab[i].0 + ab[i].1, i + 1));
    }

    list.sort_by(|a, b| (b.0 * a.1).cmp(&(a.0 * b.1)).then(a.2.cmp(&b.2)));

    for (_, _, i) in list {
        print!("{} ", i);
    }
}
