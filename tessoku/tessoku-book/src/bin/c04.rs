use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let mut ans = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ans.push(i);
            ans.push(n / i);
        }
        i += 1;
    }
    ans.sort();

    for e in ans {
        println!("{}", e);
    }
}
