use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.insert(0, 0);

    let mut ans = 0;
    for i in 1..n {
        for j in i + 1..=n {
            if a[i].min(a[j]) == i && a[i].max(a[j]) == j {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
