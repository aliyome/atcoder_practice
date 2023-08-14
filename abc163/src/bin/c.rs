use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1]
    };

    let mut ans = vec![0; n];
    for i in 0..n - 1 {
        ans[a[i] - 1] += 1;
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
