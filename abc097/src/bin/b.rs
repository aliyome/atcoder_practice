use proconio::input;

fn main() {
    input! {
        x: u32,
    };

    let mut ans = 0;
    for i in 1..1000u32 {
        for j in 2..1000u32 {
            let a = i.pow(j) as u32;
            if a > x {
                break;
            }
            if a <= x {
                ans = ans.max(a);
            }
        }
    }

    println!("{}", ans);
}
