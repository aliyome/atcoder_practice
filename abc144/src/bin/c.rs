use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut min = 10usize.pow(13);
    for i in 1..=10usize.pow(7) {
        if n % i != 0 {
            continue;
        }
        let j = n / i;
        min = min.min(i + j - 2);
    }

    println!("{}", min);
}
