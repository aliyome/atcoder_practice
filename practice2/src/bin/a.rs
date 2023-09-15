use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        tuv: [(usize, usize, usize); q],
    };

    let mut dsu = Dsu::new(n + 1);
    for &(t, u, v) in &tuv {
        if t == 0 {
            dsu.merge(u, v);
        } else {
            println!("{}", if dsu.same(u, v) { 1 } else { 0 });
        }
    }
}
