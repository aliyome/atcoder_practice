use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    };

    let mut ans = 0;
    for hh in h {
        if hh > ans {
            ans = ans.max(hh);
        } else {
            break;
        }
    }
    println!("{}", ans);
}
