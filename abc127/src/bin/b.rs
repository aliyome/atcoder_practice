use proconio::input;

fn main() {
    input! {
        r: usize,
        d: usize,
        mut x: usize
    };

    for _ in 1..=10 {
        let ans = r * x - d;
        println!("{}", ans);
        x = ans;
    }
}
