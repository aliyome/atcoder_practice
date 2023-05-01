use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    // 全探索
    for i in 1..=n - 2 {
        for j in i..=n - 1 {
            for k in j..n {
                println!("{} {} {}", i, j, k);
                if is_ok(i - 1, j - 1, k - 1, &a) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}

fn is_ok(i: usize, j: usize, k: usize, a: &[usize]) -> bool {
    if a[i] == a[j] || a[i] == a[k] || a[j] == a[k] {
        println!("same {} {} {}", a[i], a[j], a[k]);
        return false;
    }
    true
}
