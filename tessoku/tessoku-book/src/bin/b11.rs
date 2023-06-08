use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n],
      q: usize,
      x: [usize; q]
    };

    a.sort();

    for x in x {
        let mut ok = 0;
        let mut ng = n;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if a[mid] < x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok + 1);
    }
}
