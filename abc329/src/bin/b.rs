use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();

    let first = a[0];
    for a in a {
        if a != first {
            println!("{}", a);
            return;
        }
    }
}
