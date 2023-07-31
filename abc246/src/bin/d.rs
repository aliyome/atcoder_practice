use proconio::input;

fn main() {
    input! {
        n: isize,
    };

    if n == 0 {
        println!("0");
        return;
    }

    let f = |a: isize, b: isize| a * a * a + a * a * b + a * b * b + b * b * b;

    let mut min = std::isize::MAX;
    for a in 1..=10isize.pow(6) {
        let mut ng = -1isize;
        let mut ok = 10isize.pow(6);

        while (ok - ng).abs() > 1 {
            let b = (ok + ng) / 2;
            let x = f(a, b);
            if n <= x {
                ok = b;
            } else {
                ng = b;
            }
        }
        min = min.min(f(a, ok));
    }

    println!("{}", min);
}
