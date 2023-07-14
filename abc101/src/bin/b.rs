use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut s = digit_sum(n);
    if n % s == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
fn digit_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
