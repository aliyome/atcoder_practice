use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let legends = [
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ];

    for i in 0..10 {
        if s == legends[i].0 {
            println!("{}", legends[i].1);
            return;
        }
    }
}
