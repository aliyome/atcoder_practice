use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    };

    let mut ng = 0usize;
    let mut ok = 1_000_000_005;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut a_cnt = 0usize;
        let mut b_cnt = 0usize;
        for &a in &a {
            if a <= mid {
                a_cnt += 1;
            }
        }
        for &b in &b {
            if b >= mid {
                b_cnt += 1;
            }
        }
        if b_cnt <= a_cnt {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
