use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    };

    let mut ans = 0;
    for tri in (0..n).combinations(3) {
        let (x1, y1) = xy[tri[0]];
        let (x2, y2) = xy[tri[1]];
        let (x3, y3) = xy[tri[2]];
        let area = (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3);
        if area != 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
