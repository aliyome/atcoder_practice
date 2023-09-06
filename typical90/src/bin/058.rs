use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    };

    let mut x = n;
    let mut used = vec![-1; 100000];
    let mut rest = 0; // 初期値をkに設定
    for i in 0..k {
        if used[x] != -1 {
            let interval = i - used[x] as usize;
            rest = (k - i) % interval;
            break;
        }
        used[x] = i as isize;
        x = f(x);
    }

    for _ in 0..rest {
        x = f(x);
    }

    println!("{}", x);
}

fn digit_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn f(x: usize) -> usize {
    (x + digit_sum(x)) % 100000
}
