use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [isize; n],
      q: usize,
      lr: [(usize, usize); q]
    }

    let mut ok = vec![0; n + 1];
    let mut ng = vec![0; n + 1];
    for i in 1..=n {
        ok[i] = ok[i - 1] + if a[i - 1] == 1 { 1 } else { 0 };
        ng[i] = ng[i - 1] + if a[i - 1] == 0 { 1 } else { 0 };
    }

    for (l, r) in lr {
        let ok = ok[r] - ok[l - 1];
        let ng = ng[r] - ng[l - 1];
        if ok > ng {
            println!("win");
        } else if ok < ng {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
