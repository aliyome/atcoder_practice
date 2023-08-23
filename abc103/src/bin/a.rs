use proconio::input;

fn main() {
    input! {
        a: [isize; 3],
    };

    let mut ans = 10000000;
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            for k in 0..3 {
                if i == k || j == k {
                    continue;
                }
                ans = ans.min((a[k] - a[j]).abs() + (a[j] - a[i]).abs());
            }
        }
    }
    println!("{}", ans);
}
