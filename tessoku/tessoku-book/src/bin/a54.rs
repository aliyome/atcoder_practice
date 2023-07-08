use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
          t: usize
        }
        match t {
            1 => {
                input! {
                  x: String,
                  y: usize,
                }
                map.insert(x, y);
            }
            2 => {
                input! {
                  x: String,
                }
                println!("{}", map.get(&x).unwrap());
            }
            _ => unreachable!(),
        }
    }
}
