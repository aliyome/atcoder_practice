use proconio::input;

fn main() {
    input! {
      n: usize,
      mut lr: [(usize, usize); n ]
    }

    lr.sort_by(|a, b| a.1.cmp(&b.1));

    let mut count = 0;
    let mut curr = 0;
    for &(l, r) in &lr {
        if curr <= l {
            curr = r;
            count += 1;
        }
    }

    println!("{}", count);
}
