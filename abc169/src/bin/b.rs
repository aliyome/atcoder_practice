use num_bigint::BigUint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    if a.contains(&0) {
        println!("0");
        return;
    }

    // let mut ans = BigUint::from(1usize);
    // for a in a {
    //     ans *= a;
    //     if ans.to_string() > "1000000000000000000".to_string() {
    //         println!("-1");
    //         return;
    //     }
    // }
    // println!("{}", ans);

    let mut ans = 1;
    for a in a {
        if ans > 10usize.pow(18) / a {
            println!("-1");
            return;
        }
        ans *= a;
    }
    println!("{}", ans);
}
