use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = i + 1;
    }

    let mut reversed = false;
    for _ in 0..q {
        input! {
          t: usize
        }

        if t == 1 {
            input! {
              x: usize,
              y: usize
            }
            if reversed {
                a[n - x] = y;
            } else {
                a[x - 1] = y;
            }
        } else if t == 2 {
            reversed = !reversed;
            // a.reverse();
        } else {
            input! {
              x: usize,
            }
            if reversed {
                println!("{}", a[n - x]);
            } else {
                println!("{}", a[x - 1]);
            }
        }
    }
}
