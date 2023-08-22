use proconio::input;

fn main() {
    input! {
        n: usize,
        t: isize,
        a: isize,
        h: [isize; n]
    };

    let mut temp = vec![0; n];
    for i in 0..n {
        temp[i] = (1000 * a - (1000 * t - 6 * h[i])).abs();
    }

    let mut min_i = 0;
    let mut min = 10isize.pow(10);
    for i in 0..n {
        if temp[i] < min {
            min = temp[i];
            min_i = i;
        }
    }

    println!("{}", min_i + 1);
}
