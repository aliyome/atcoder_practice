use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    };

    let mut a = a;
    let mut b = b;

    if a == 1 {
        a += 13;
    }
    if b == 1 {
        b += 13;
    }

    if a == b {
        println!("Draw");
    } else if a > b {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
