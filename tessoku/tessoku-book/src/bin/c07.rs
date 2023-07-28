use proconio::input;

fn main() {
    input! {
      n: usize,
      mut c: [usize; n],
      q: usize,
      x: [usize; q]
    }

    c.sort();

    for &x in &x {
        let mut sum = 0;
        let mut count = 0;
        for &c in &c {
            sum += c;
            if sum > x {
                break;
            }
            count += 1;
        }
        println!("{}", count);
    }
}
