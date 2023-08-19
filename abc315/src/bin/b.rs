use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m]
    };

    let sum = d.iter().sum::<usize>();
    let mut x = (sum + 1) / 2;
    for i in 0..m {
        if x <= d[i] {
            println!("{} {}", i + 1, x);
            return;
        }
        x -= d[i];
    }
}
