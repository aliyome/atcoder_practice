use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut min = 0;
    let mut current = 0;

    for &change in &a {
        current += change;
        if current < min {
            min = current;
        }
    }

    let initial = if min < 0 { -min } else { 0 };

    let result = initial + current;
    println!("{}", result);
}
