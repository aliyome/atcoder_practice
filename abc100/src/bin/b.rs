use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    };
    // 100で割り切れる -> x % 100 == 0
    // ちょうどD回割り切れる -> x % 100 == 0; x /= 100; がD回続く

    let mut count = 0;
    for x in 1.. {
        if f(x) == d {
            count += 1;
        }
        if count == n {
            println!("{}", x);
            return;
        }
    }
}

fn f(x: usize) -> usize {
    if x % 100 != 0 {
        return 0;
    } else {
        return f(x / 100) + 1;
    }
}
