use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };

    let mut list = vec![];
    for i in 1..=100 {
        if a % i == 0 && b % i == 0 {
            list.push(i);
        }
    }

    println!("{}", list[list.len() - k]);
}
