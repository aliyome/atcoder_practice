use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }
    if h == 1 || w == 1 {
        println!("{}", w * h);
    } else {
        let ww = (w + 1) / 2;
        let hh = (h + 1) / 2;
        println!("{}", ww * hh);
    }
}
