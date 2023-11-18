use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut cnt = 0;
    for i in 0..n {
        let m = i + 1;
        for d in 1..=d[i] {
            let s = format!("{}{}", m, d);
            let first = s.chars().nth(0).unwrap();
            if s.chars().all(|c| c == first) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
