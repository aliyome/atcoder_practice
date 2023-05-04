use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c : usize,
        d : usize,
        e: usize,
        f: usize,
        x: usize
    };

    let calc = |a: usize, b: usize, c: usize| {
        let r = x / (a + c);
        let s = x % (a + c);
        (r * a + a.min(s)) * b
    };

    let takahashi = calc(a, b, c);
    let aoki = calc(d, e, f);
    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
