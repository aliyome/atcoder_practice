use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let has_adjacent_ab = s.chars().collect::<Vec<char>>().windows(2).any(|window| {
        (window[0] == 'a' && window[1] == 'b') || (window[0] == 'b' && window[1] == 'a')
    });

    if has_adjacent_ab {
        println!("Yes");
    } else {
        println!("No");
    }
}
