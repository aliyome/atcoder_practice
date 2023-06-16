use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.insert(0, 0);

    let mut ans: usize = 0;
    // case1: ai = i && aj = j && i < j となる個数
    let mut count = 0;
    for i in 1..=n {
        if a[i] == i {
            count += 1;
        }
    }
    ans += count * (count - 1) / 2;

    // case2: ai = j && aj = i && i < j となる個数
    for i in 1..n {
        let j = a[i];
        if i < j && a[j] == i {
            ans += 1;
        }
    }
    println!("{}", ans);
}
