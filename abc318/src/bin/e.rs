use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut prefix_sum = HashMap::new();
    let mut suffix_sum = HashMap::new();
    let mut ans = 0;

    for &x in a.iter() {
        *suffix_sum.entry(x).or_insert(0) += 1;
    }

    for i in 0..n {
        let current_element = a[i];
        if let Some(count) = suffix_sum.get_mut(&current_element) {
            *count -= 1;
        }
        for (&x, &count_prefix) in prefix_sum.iter() {
            if x != current_element {
                if let Some(&count_suffix) = suffix_sum.get(&x) {
                    ans += count_prefix * count_suffix;
                }
            }
        }
        *prefix_sum.entry(current_element).or_insert(0) += 1;
    }

    println!("{}", ans);
}
