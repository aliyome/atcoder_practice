use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    for i in 1usize.. {
        ans += i;
        if ans >= n {
            println!("{}", i);
            return;
        }
    }
}
