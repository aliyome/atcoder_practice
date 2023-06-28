use proconio::input;

fn main() {
    input! {
      n: usize, // <= 100000
      _x: usize, // = 2
      _y: usize, // = 3
      a: [usize; n] // <= 10^18
    }

    // Grundy数
    let mut ans = 0;
    for i in 0..n {
        match a[i] % 5 {
            0 | 1 => {
                ans ^= 0;
            }
            2 | 3 => {
                ans ^= 1;
            }
            4 => {
                ans ^= 2;
            }
            _ => {}
        }
    }

    if ans != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
