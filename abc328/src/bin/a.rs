use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }

    println!("{}", s.iter().filter(|&&s| s <= x).sum::<usize>());
}
