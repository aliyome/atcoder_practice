use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ab: [(usize, usize); n]
    }

    let mut v = vec![];
    for i in 0..n {
        v.push(ab[i].1);
        v.push(ab[i].0 - ab[i].1);
    }

    v.sort();

    let mut ans = 0;
    for _ in 0..k {
        ans += v.pop().unwrap();
    }

    println!("{}", ans);
}
