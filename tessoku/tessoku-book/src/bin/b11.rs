use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n],
      q: usize,
      x: [usize; q]
    };

    a.sort();

    let is_ok = |index: usize, key: usize| a[index] >= key;

    for x in x {
        let mut left = -1;
        let mut right = n as isize;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if is_ok(mid as usize, x) {
                right = mid;
            } else {
                left = mid;
            }
        }
        println!("{}", right);
    }
}
