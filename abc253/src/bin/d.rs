use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // <=10^9
        a: usize, // <=10^9
        b: usize, // <=10^9
    };

    if a == 1 || b == 1 {
        println!("0");
        return;
    }

    // 1からNまでの総和
    let sum_n = n * (n + 1) / 2;
    // println!("{}", sum_n);

    // Aの倍数の総和
    let sum_a = (n / a) * (a + (n / a) * a) / 2;
    // println!("{}", sum_a);

    // Bの倍数の総和
    let sum_b = (n / b) * (b + (n / b) * b) / 2;
    // println!("{}", sum_b);

    // AとBの公倍数の総和
    let lcm_ab = lcm(a, b);
    let sum_ab = (n / lcm_ab) * (lcm_ab + (n / lcm_ab) * lcm_ab) / 2;

    println!("{}", sum_n - sum_a - sum_b + sum_ab);

    // let mut set = BTreeSet::new();
    // for i in 1..=n {
    //     set.insert(i);
    // }

    // let mut set_ab = BTreeSet::new();
    // for i in (a..=n).step_by(a) {
    //     set_ab.insert(i);
    // }
    // for i in (b..=n).step_by(b) {
    //     set_ab.insert(i);
    // }

    // println!("{}", set.difference(&set_ab).into_iter().sum::<usize>());
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
