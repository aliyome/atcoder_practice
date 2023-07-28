use proconio::input;

fn main() {
    input! {
        n: usize, // <= 10^5
        mut a: [isize; n]
    };

    // 1-indexed
    a.sort();
    a.insert(0, 0);

    let mut acc = vec![0; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + a[i];
    }

    let mut ans = 0;
    for i in 1..n {
        ans += (acc[n] - acc[i]) - (n as isize - i as isize) * a[i];
    }

    println!("{}", ans);
}
