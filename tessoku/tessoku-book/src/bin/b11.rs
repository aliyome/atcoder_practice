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
        let mut low = 0;
        let mut high = n;
        while low != high {
            let mid = (low + high) / 2;
            if a[mid] < x {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        println!("{}", low);
    }
}
