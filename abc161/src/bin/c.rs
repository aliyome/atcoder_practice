use proconio::input;

fn main() {
    input! {
        n: isize,
        k: isize
    };

    // 全探索 O(10^18)
    // let mut ans = n;
    // while (ans - k).abs() < ans {
    //     ans = (ans - k).abs();
    // }

    let mut ans = n;
    ans %= k;
    while (ans - k).abs() < ans {
        ans = (ans - k).abs();
    }

    println!("{}", ans);
}
