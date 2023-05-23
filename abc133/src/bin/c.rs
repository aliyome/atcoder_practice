use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize
    };

    let mut ans = 3000;
    for i in l..=(l + 2019) {
        if l > r {
            break;
        }
        for j in i + 1..=(l + 2019) {
            if j > r {
                break;
            }
            if i == j {
                continue;
            }

            ans = ans.min(i * j % 2019);
        }
    }

    println!("{}", ans);
}
