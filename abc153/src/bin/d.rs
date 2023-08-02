use proconio::input;

fn main() {
    input! {
        mut h: usize,
    };

    let mut ans = 0usize;
    let mut enemies = 1usize;

    while h > 0 {
        ans += enemies;
        h /= 2;
        enemies *= 2;
    }

    println!("{}", ans);
}
