use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let max_value = *a.iter().max().unwrap();

    let second_max = a.iter().filter(|&&x| x != max_value).max().unwrap();

    println!("{}", second_max);
}
