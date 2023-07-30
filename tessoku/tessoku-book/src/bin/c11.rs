use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      a: [usize; n]
    }

    let calc_seats = |mid| a.iter().map(|&x| (x as f64 / mid) as usize).collect_vec();
    let total_seats = |seats: Vec<usize>| seats.iter().sum::<usize>();
    let is_ok = |mid| total_seats(calc_seats(mid)) <= k;

    let mut ok = 1_000_000_000.0f64;
    let mut ng = 0.0f64;
    while (ok - ng) > 10.0f64.powi(-6) {
        let mid = (ok + ng) / 2.0;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let ans = calc_seats(ok);

    for &a in &ans {
        print!("{} ", a);
    }
}
