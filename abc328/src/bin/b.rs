use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let mut cnt = 0;
    for i in 0..n {
        let d = d[i];
        let i = i + 1;
        for j in 1..=d {
            let s = format!("{}{}", i, j);
            let first = s.chars().nth(0).unwrap();
            if s.chars().all(|c| c == first) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
