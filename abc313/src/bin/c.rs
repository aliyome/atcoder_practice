use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let sum = a.iter().sum::<isize>();
    let avg = sum / n as isize;
    let rest = sum - avg * n as isize;
    let mut b = vec![avg; n];
    for i in 0..rest {
        b[i as usize] += 1;
    }
    a.sort();
    a.reverse();

    let mut diffs = vec![0; n];
    let mut diff = 0;
    for i in 0..n {
        diffs[i] = (a[i] - b[i]).abs();
        diff += diffs[i];
    }

    println!("{}", diff / 2);
}
