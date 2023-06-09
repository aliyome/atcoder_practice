use proconio::input;

fn main() {
    input! {
      n: usize,
      ta: [(char, usize); n],
    }

    let mut ans = 0;
    for i in 0..n {
        let (t, a) = ta[i];

        // ans -= a で負の数にならないようにする
        ans += 10000;

        match t {
            '+' => ans += a,
            '-' => ans -= a,
            '*' => ans *= a,
            _ => unreachable!(),
        }

        ans %= 10000;
        println!("{}", ans);
    }
}
