use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut abc: [usize; 3]
    }

    let mut ans = std::usize::MAX;
    for i in 0..=9999 {
        for j in 0..=9999 - i {
            for k in 0..=9999 - i - j {
                if i * abc[0] + j * abc[1] + k * abc[2] == n {
                    ans = ans.min(i + j + k);
                }
            }
        }
    }
    println!("{}", ans);
}
