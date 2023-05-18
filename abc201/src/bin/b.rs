use proconio::input;

fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n]
    };

    st.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    println!("{}", st[1].0);
}
