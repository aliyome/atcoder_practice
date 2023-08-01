use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("0");
        return;
    }

    let mut x = 2;
    for k in 1..=n {
        x *= 2;
        if x > n {
            println!("{}", k);
            return;
        }
    }
}
