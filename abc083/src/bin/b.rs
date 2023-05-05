use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    };

    let mut total = 0;
    for n in 1..=n {
        let mut sum = 0;
        let mut m = n;
        while m > 0 {
            sum += m % 10;
            m /= 10;
        }
        if a <= sum && sum <= b {
            total += n;
        }
    }

    println!("{}", total);
}
