use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    // 3^38 > 10^18
    // 5^26 > 10^18

    for i in 0..38 {
        for j in 0..26 {
            let x = 3_u128.pow(i) + 5_u128.pow(j);
            if x == n as u128 {
                println!("{} {}", i, j);
                return;
            }
            if x > n as u128 {
                break;
            }
        }
    }
    println!("-1");
}
