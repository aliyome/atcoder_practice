use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let first = a[0];

    if a.iter().all(|&x| x == first) {
        println!("Yes");
    } else {
        println!("No");
    }
}
