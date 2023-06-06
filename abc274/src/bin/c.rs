use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut rev = HashMap::new();
    let mut counts = vec![0; 2 * n + 3];

    // O(N)
    for i in 0..n {
        let a = a[i];
        let next_1 = (i + 1) * 2;
        let next_2 = (i + 1) * 2 + 1;

        rev.insert(next_1, a);
        rev.insert(next_2, a);
        counts[next_1] = counts[a] + 1;
        counts[next_2] = counts[a] + 1;
    }

    // O(N^2)
    for k in 1..=2 * n + 1 {
        println!("{}", counts[k]);
    }
}
