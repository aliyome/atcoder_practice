use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        _m: usize,
    };

    let mut list = vec![BTreeSet::new(); n];
    let mut price_list = vec![0; n];
    for i in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c]
        };
        let mut set = BTreeSet::new();
        for &f in &f {
            set.insert(f);
        }
        list[i] = set;
        price_list[i] = p;
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            // Pi≥Pj.
            if price_list[i] < price_list[j] {
                continue;
            }
            // The j-th product has all functions of the i-th product.
            if !list[i].is_subset(&list[j]) {
                continue;
            }
            // Pi>Pj, or the j-th product has one or more functions that the i-th product lacks.
            if price_list[i] > price_list[j] || list[j].len() > list[i].len() {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
