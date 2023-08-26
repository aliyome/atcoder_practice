use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        p: [usize; a],
        q: [usize; b]
    };

    let mut pin = vec!['x'; 11];
    for &p in &p {
        pin[p] = '.';
    }
    for &q in &q {
        pin[q] = 'o';
    }
    println!("{} {} {} {}", pin[7], pin[8], pin[9], pin[0]);
    println!(" {} {} {}", pin[4], pin[5], pin[6]);
    println!("  {} {}", pin[2], pin[3]);
    println!("   {}", pin[1]);
}
