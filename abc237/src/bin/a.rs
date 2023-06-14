use proconio::input;

fn main() {
    input! {
        n: isize,
    };

    if -2isize.pow(31) <= n && n < 2isize.pow(31) - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
