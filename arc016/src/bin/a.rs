use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    if m > 1 {
        println!("1");
        return;
    }
    for i in 2..=n {
        if i != m {
            println!("{}", i);
            return;
        }
    }
}
