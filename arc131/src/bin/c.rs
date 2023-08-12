use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // 4x10^5
        a: [usize; n] // 10^9, すべて異なる, すべてのXORは0ではない
    };

    if n == 1 {
        println!("Win");
        return;
    }

    let mut set = HashSet::new();
    for &a in &a {
        set.insert(a);
    }

    let mut xor = 0;
    for i in 0..n {
        xor ^= a[i];
    }

    let mut count_zero = 0;
    if xor == 0 {
        count_zero += 1;
    }

    for i in 1..n {
        xor ^= a[i - 1];
        xor ^= a[i];

        let target = xor ^ 0;
        if set.contains(&target) {
            count_zero += 1;
        }
    }

    if count_zero >= 2 {
        println!("Lose");
    } else {
        println!("Win");
    }
}
