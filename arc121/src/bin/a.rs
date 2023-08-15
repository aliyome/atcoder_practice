use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    };

    let mut xy = xy.into_iter().enumerate().collect_vec();

    let mut list = vec![];

    let dist = |a: (isize, isize), b: (isize, isize)| (a.0 - b.0).abs().max((a.1 - b.1).abs());

    // X最大 - X最小
    xy.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
    list.push((
        dist(xy[xy.len() - 1].1, xy[0].1),
        (xy[xy.len() - 1].0, xy[0].0),
    ));
    list.push((
        dist(xy[xy.len() - 2].1, xy[0].1),
        (xy[xy.len() - 2].0, xy[0].0),
    ));
    list.push((
        dist(xy[xy.len() - 1].1, xy[1].1),
        (xy[xy.len() - 1].0, xy[1].0),
    ));

    // Y最大 - Y最小
    xy.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));
    list.push((
        dist(xy[xy.len() - 1].1, xy[0].1),
        (xy[xy.len() - 1].0, xy[0].0),
    ));
    list.push((
        dist(xy[xy.len() - 2].1, xy[0].1),
        (xy[xy.len() - 2].0, xy[0].0),
    ));
    list.push((
        dist(xy[xy.len() - 1].1, xy[1].1),
        (xy[xy.len() - 1].0, xy[1].0),
    ));

    // 先頭から２番目
    list.sort_by(|a, b| b.0.cmp(&a.0));
    let mut count = 0;
    let mut ii = 10000000000;
    let mut jj = 10000000000;
    for &(d, (i, j)) in &list {
        if (ii == i && jj == j) || (ii == j && jj == i) {
            continue;
        }
        ii = i;
        jj = j;

        count += 1;
        if count == 2 {
            println!("{}", d);
            return;
        }
    }
}
