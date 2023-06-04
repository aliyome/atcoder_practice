use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [isize; m]
    };

    x.sort();

    let mut dists = vec![0; m - 1];
    for i in 0..m - 1 {
        dists[i] = (x[i + 1] - x[i]).abs();
    }

    dists.sort();
    dists.reverse();

    let ans: isize = dists.iter().skip(n - 1).sum();

    println!("{}", ans);
}
