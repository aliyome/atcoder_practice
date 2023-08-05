use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [isize; n]
    };
    let max = *p.iter().max().unwrap();
    let mut max_count = 0;
    for &p in &p {
        if max == p {
            max_count += 1;
        }
    }

    if max == p[0] && max_count > 1 {
        println!("1");
    } else if max == p[0] {
        println!("0");
    } else {
        println!("{}", max - p[0] + 1);
    }
}
