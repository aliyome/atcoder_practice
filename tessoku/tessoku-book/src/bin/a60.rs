use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n]
    }

    a.insert(0, 0);

    // O(N^2) TLE
    for i in 1..=n {
        let mut d = -1isize;
        for j in (0..i).rev() {
            if a[i] < a[j] {
                d = j as isize;
                break;
            }
        }
        print!("{} ", d);
    }
}
