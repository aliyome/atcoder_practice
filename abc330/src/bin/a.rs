use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        a: [i32; n],
    }

    let count = a.iter().filter(|&&score| score >= l).count();

    println!("{}", count);
}
