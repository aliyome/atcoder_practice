use proconio::input;

fn main() {
    input! {
        x: usize,
        k: u32,
    };

    let mut ans = x;
    for i in 1..=k {
        let tmp = ans % 10usize.pow(i);
        ans -= tmp;
        if 5 <= tmp / 10usize.pow(i - 1) {
            ans += 10usize.pow(i);
        }
    }
    println!("{}", ans);
}
