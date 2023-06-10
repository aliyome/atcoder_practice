use proconio::input;

fn main() {
    input! {
        n: isize,
    };

    let mut min: isize = 1000000000;
    let mut dist: isize = 100000000;
    for i in 0..=20 {
        let diff = (n - (i * 5)).abs();
        if diff < min {
            min = diff;
            dist = i * 5;
        }
    }

    println!("{}", dist);
}
