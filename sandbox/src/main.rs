fn main() {
    let s = "hogehoge".chars().collect::<Vec<char>>();
    let n = s.len();

    if n % 2 > 0 {
        println!("No");
    }

    let x = &s[0..n / 2];
    let y = &s[n / 2..];

    if x == y {
        println!("Yes");
    }
}
