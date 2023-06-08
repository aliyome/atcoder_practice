use proconio::input;

fn main() {
    input! {
      n: usize,
      x: usize,
      a: [usize; n]
    }

    let mut ng = -1isize;
    let mut ok = n as isize - 1;
    let is_ok = |mid, key| key <= a[mid];

    while (ok - ng).abs() > 1 {
        let mid = (ng + ok) / 2;
        if is_ok(mid as usize, x) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}
