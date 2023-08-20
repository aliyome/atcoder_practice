use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut books = vec![vec![]];
    for _ in 0..n {
        input! {
            c: usize,
            p: [usize; c]
        };
        books.push(p);
    }

    let mut routes = vec![];
    dfs(&books, 1, &mut routes, &mut vec![false; n + 1]);

    for &v in &routes {
        print!("{} ", v);
    }
}

fn dfs(books: &Vec<Vec<usize>>, i: usize, routes: &mut Vec<usize>, visited: &mut Vec<bool>) {
    if visited[i] {
        return;
    }
    visited[i] = true;
    for &j in books[i].iter() {
        if visited[j] {
            continue;
        }
        dfs(books, j, routes, visited);
        routes.push(j);
    }
}
