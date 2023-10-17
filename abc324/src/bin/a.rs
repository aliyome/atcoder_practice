use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if a.iter().all(|&x| x == a[0]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
