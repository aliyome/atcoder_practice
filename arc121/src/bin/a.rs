use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n]
    };

    xy.sort_by(|a, b| a.0.cmp(&b.0));

    let mut list = vec![];
    list.push((xy[xy.len() - 1].0 - xy[0].0).abs());
    list.push((xy[xy.len() - 2].0 - xy[0].0).abs());
    list.push((xy[xy.len() - 1].0 - xy[1].0).abs());

    xy.sort_by(|a, b| a.1.cmp(&b.1));
    list.push((xy[xy.len() - 1].1 - xy[0].1).abs());
    list.push((xy[xy.len() - 2].1 - xy[0].1).abs());
    list.push((xy[xy.len() - 1].1 - xy[1].1).abs());

    list.sort_by(|a, b| b.cmp(&a));
    println!("{}", list[1]);
}
