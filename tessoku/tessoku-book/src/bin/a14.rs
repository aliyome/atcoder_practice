use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
      n: usize,
      k: isize,
      mut a: [isize; n],
      mut b: [isize; n],
      mut c: [isize; n],
      mut d: [isize; n],
    }

    // 半分全列挙
    let mut ab = Vec::new();
    let mut cd = Vec::new();
    for &a in &a {
        for &b in &b {
            ab.push(a + b);
        }
    }
    for &c in &c {
        for &d in &d {
            cd.push(c + d);
        }
    }
    ab.sort();
    cd.sort();

    // O(N^2 logN)
    for &ab in &ab {
        let mut ng = 0;
        let mut ok = cd.len();
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if cd[mid] == k - ab {
                println!("Yes");
                return;
            }
            if k <= ab + cd[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
    }

    println!("No");
}
