use proconio::input;

fn main() {
    input! {
      n: usize,
      k: isize,
      mut ab: [(isize, isize); n]
    }

    let mut ans = 0;
    for a_min in 0..=100 - k {
        let a_max = a_min + k;
        for b_min in 0..=100 - k {
            let b_max = b_min + k;
            let count = ab
                .iter()
                .filter(|(a, b)| a_min <= *a && *a <= a_max && b_min <= *b && *b <= b_max)
                .count();
            ans = ans.max(count);
        }
    }
    println!("{}", ans);
}
