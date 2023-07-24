use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n]
    };

    if n <= k {
        println!("0");
        return;
    }

    h.sort();
    h.reverse();
    let ans: usize = h[k..].iter().sum();
    println!("{}", ans);
}
