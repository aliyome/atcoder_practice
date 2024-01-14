use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // 0からNまでの各値に対してx, y, zを全探索する
    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x + y + z <= n {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}
