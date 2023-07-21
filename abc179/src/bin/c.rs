use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = vec![];
    for a in 1..=n {
        for b in 1..=n {
            if a * b > n {
                break;
            }
            let c = n as isize - a as isize * b as isize;
            if c > 0 {
                ans.push((a, b, c as usize));
            }
        }
    }

    println!("{}", ans.len());
}
