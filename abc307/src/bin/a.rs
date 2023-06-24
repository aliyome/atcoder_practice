use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 7]
    };

    for i in 0..n {
        let mut sum = 0;
        for j in 0..7 {
            sum += a[i * 7 + j];
        }
        print!("{} ", sum);
    }
}
