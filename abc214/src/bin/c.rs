use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n]
    };

    let mut prev = t[0];
    println!("{}", prev);
    for i in 1..n {
        let ans = t[i].min(prev + s[i - 1]);
        println!("{}", ans);
        prev = ans;
    }
}
