use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    };

    // a < b のとき
    // aa = 0 - A -1 までの xor
    // bb = 0 - B までの xor
    // cc = A - B までの xor とすると
    // aa xor cc = bb となる
    // 式を変形すると cc = aa xor bb となる
    let xor_sum = |x: i64| {
        let x = x + 1;
        // 10^12 <= 2^40 なので 40bit で十分
        let mut ans = 0i64;
        // 各桁の 1 の数を数える
        for i in 0..50 {
            // 周期
            let loops: i64 = 1 << (i + 1);
            // 1 の数
            let mut cnt: i64 = (x / loops) * (loops / 2);
            cnt += 0i64.max((x % loops) - (loops / 2));
            // 1 の数が奇数なら 1 を立てる
            if cnt % 2 == 1 {
                ans += 1 << i;
            }
        }
        ans
    };
    // 0, 1, 2, ..., x の xor は
    // 1(0,1) xor 1(2,3) xor ... となる
    // x が偶数のとき、 1 xor ... xor 1 xor x となる
    // x が奇数のとき、 1 xor ... xor 1(x-1, x) となる
    let xor_sum_2 = |x| {
        let cnt1 = (x + 1) / 2; // 切り上げ
        let ans = cnt1 % 2;
        if x % 2 == 0 {
            ans ^ x
        } else {
            ans
        }
    };

    // let aa = xor_sum(a - 1);
    // let bb = xor_sum(b);
    let aa = xor_sum_2(a - 1);
    let bb = xor_sum_2(b);
    let ans = aa ^ bb;
    println!("{}", ans);
}
