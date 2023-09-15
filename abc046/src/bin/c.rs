use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(usize, usize); n]
    };

    let (mut c_t, mut c_a) = (0usize, 0usize);
    for &(t, a) in &ta {
        for i in 1.. {
            if t * i < c_t || a * i < c_a {
                continue;
            }
            c_t = t * i;
            c_a = a * i;
            break;
        }
    }
    println!("{}", c_t + c_a);
}
