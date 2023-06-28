use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
    }

    let move_count = n * 2 - 2;
    if move_count <= k && k % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
