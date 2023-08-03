use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [usize; n]
    };

    let mut count = 1;
    let mut d = 0;
    for i in 0..n {
        d += l[i];
        if d <= x {
            count += 1;
        }
    }

    println!("{}", count);
}
