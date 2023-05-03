use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        dcs: [(usize, usize, usize); n]
    }

    //
    // 全探索 O(N!) 2点
    //
    let mut max = 0;
    // 全組み合わせを列挙
    for indices in (0..n).permutations(n) {
        let mut sum = 0;
        let mut day = 0;
        for i in indices {
            let (d, c, s) = dcs[i];
            day += c;
            if day > d {
                break;
            }
            sum += s;
        }
        max = max.max(sum);
    }

    println!("{}", max);
}
