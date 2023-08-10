use proconio::input;

fn main() {
    input! {
        p: [usize; 26],
    };

    for &p in &p {
        print!("{}", (b'a' + p as u8 - 1) as char);
    }
}
