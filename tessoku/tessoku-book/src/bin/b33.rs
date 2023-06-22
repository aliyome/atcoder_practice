use proconio::input;

fn main() {
    input! {
      n: usize,
      _h: usize,
      _w: usize,
      ab: [(usize, usize); n]
    }

    // Nim の亜種
    let mut can_move_count = vec![];
    for i in 0..n {
        let (a, b) = ab[i];
        can_move_count.push(a - 1);
        can_move_count.push(b - 1);
    }

    let xor = can_move_count.iter().fold(0, |acc, x| acc ^ x);

    if xor == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
