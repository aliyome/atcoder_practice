use proconio::input;

fn main() {
    input! {
      d: usize,
      x: usize,
      mut a: [isize; d - 1],
      q: usize,
      st: [(usize, usize); q]
    }

    let mut price = vec![0; d];
    price[0] = x as isize;
    for i in 0..d - 1 {
        price[i + 1] = price[i] + a[i];
    }

    for &(s, t) in &st {
        if price[s - 1] < price[t - 1] {
            println!("{}", t);
        } else if price[s - 1] > price[t - 1] {
            println!("{}", s);
        } else {
            println!("Same");
        }
    }
}
