use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    if [1, 2, 3].contains(&n) {
        println!("Yes");
        return;
    }

    let mut x = 1;
    for _ in 1..=60 {
        let mut y = 1;
        for _ in 1..=40 {
            if x * y == n {
                println!("Yes");
                return;
            }
            y *= 3;
        }
        x *= 2;
    }

    println!("No");
}
