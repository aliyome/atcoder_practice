use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m]
    };

    let mut acc = vec![0isize; n + 2];
    for &(l, r) in &lr {
        acc[l] += 1;
        acc[r + 1] -= 1;
    }

    for i in 1..=n {
        acc[i] += acc[i - 1];
    }

    let ans = acc.iter().filter(|&&x| x == m as isize).count();
    println!("{}", ans);
}
