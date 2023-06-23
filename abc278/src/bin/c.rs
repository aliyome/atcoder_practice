use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // <=10^9
        q: usize, // <=2x10^5
        tab: [(usize, usize, usize); q]
    };

    let mut set = HashSet::new();

    for &(t, a, b) in &tab {
        match t {
            1 => {
                set.insert((a, b));
            }
            2 => {
                set.remove(&(a, b));
            }
            3 => {
                if set.contains(&(a, b)) && set.contains(&(b, a)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => unreachable!(),
        }
    }
}
