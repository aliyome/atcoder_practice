use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    let mut color = vec![0; 10001];
    for i in 0..n {
        color[a[i]] += 1;
    }

    let mut ans = 0;
    for i in 1..n {
        if a[i] == a[i - 1] {
            for j in 1..10001 {
                if color[j] == 0 {
                    a[i] = j;
                    color[j] = 1;
                    ans += 1;
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
