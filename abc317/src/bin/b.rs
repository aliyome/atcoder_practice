use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut diff = vec![];

    for i in 1..n {
        diff.push(a[i] - a[i - 1]);
    }

    for i in 0..n - 1 {
        if diff[i] != 1 {
            println!("{}", a[i] + 1);
            return;
        }
    }
}
