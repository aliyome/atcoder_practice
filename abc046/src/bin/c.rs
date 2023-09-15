use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(u128, u128); n]
    };

    let (mut c_t, mut c_a) = (1u128, 1u128);
    for &(t, a) in &ta {
        // 二分探索
        let mut ok = 1e19 as u128;
        let mut ng = 0u128;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            let next_t = t * mid;
            let next_a = a * mid;
            if c_t <= next_t && c_a <= next_a {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        c_t = t * ok;
        c_a = a * ok;
    }
    println!("{}", c_t + c_a);
}
