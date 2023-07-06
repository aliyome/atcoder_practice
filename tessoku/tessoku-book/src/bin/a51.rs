use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut stack = vec![];

    for _ in 0..q {
        input! {
          t: usize
        }
        if t == 1 {
            input! {
              x: String
            }
            stack.push(x);
        } else if t == 2 {
            println!("{}", stack.last().unwrap());
        } else {
            stack.pop();
        }
    }
}
