use proconio::input;

fn main() {
    input! {
        x: usize,
    };

    let mut ans = 1;
    for i in 2..=x {
        for j in 2u32.. {
            if i.pow(j) > x {
                break;
            }
            ans = ans.max(i.pow(j));
        }
    }

    println!("{}", ans);
}
