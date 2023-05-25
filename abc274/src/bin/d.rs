use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, i64, i64),
        a: [i64; n]
    };

    let mut dp_x: Vec<HashSet<i64>> = vec![HashSet::new(); n + 1];
    let mut dp_y: Vec<HashSet<i64>> = vec![HashSet::new(); n + 1];
    dp_x[0].insert(0);
    dp_y[0].insert(0);

    for i in 0..n {
        if i % 2 == 0 {
            // x 方向
            if i == 0 {
                dp_x[i].insert(a[i]);
            } else {
                let mut dp_x_i = HashSet::new();
                for &xs in dp_x[i - 1].iter() {
                    dp_x_i.insert(xs + a[i]);
                    dp_x_i.insert(xs - a[i]);
                }
                dp_x[i] = dp_x_i;
                dp_y[i] = dp_y[i - 1].clone();
            }
        } else {
            // y 方向
            let mut dp_y_i = HashSet::new();
            for &ys in dp_y[i - 1].iter() {
                dp_y_i.insert(ys + a[i]);
                dp_y_i.insert(ys - a[i]);
            }
            dp_y[i] = dp_y_i;
            dp_x[i] = dp_x[i - 1].clone();
        }
    }

    if dp_x[n - 1].contains(&x) && dp_y[n - 1].contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
