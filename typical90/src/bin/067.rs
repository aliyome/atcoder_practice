use proconio::input;

fn main() {
    input! {
        n: String,
        k: usize
    }

    let mut d8 = n;
    for _ in 0..k {
        // 8 -> 10
        let d10 = usize::from_str_radix(&d8, 8).unwrap();
        // 10 -> 9
        let d9 = base10_to_base_x(d10, 9);
        // 9 -> 8 (8 -> 5)
        let d9 = d9
            .into_iter()
            .map(|x| if x == 8 { 5 } else { x })
            .map(|x| x.to_string())
            .collect::<String>();
        d8 = d9
    }
    println!("{}", d8);
}

fn base10_to_base_x(n: usize, base: usize) -> Vec<usize> {
    let mut s = vec![];
    let mut n = n;
    while n > 0 {
        s.push(n % base);
        n /= base;
    }
    s.reverse();
    s
}
