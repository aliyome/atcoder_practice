use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    };

    let mut ans: usize = 0;
    for i in 0..64 {
        ans += a[i] * 2usize.pow(i as u32);
    }

    println!("{}", ans);
}
