use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m]
    };

    let mut min = 1;
    let mut max = 10usize.pow(5);
    for &(l, r) in &lr {
        min = min.max(l);
        max = max.min(r);
    }

    if min > max {
        println!("0");
    } else {
        println!("{}", max as i64 - min as i64 + 1);
    }
}
