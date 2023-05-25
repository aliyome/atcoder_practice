use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    };

    let mut reachable = std::collections::HashMap::new();
    for j in 0..n {
        *reachable.entry(b[c[j] - 1]).or_insert(0) += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..n {
        let j = reachable.get(&a[i]);
        if j.is_some() {
            ans += j.unwrap();
            // println!("{} {}", i, j.unwrap());
        }
    }

    println!("{}", ans);
}
