use proconio::input;

fn main() {
    input! {
      n: usize,
      q: usize,
    }

    let mut segtree = SegmentTree::new(n, |x, y| x.max(y));

    for _ in 0..q {
        input! {
          t: usize
        }
        match t {
            1 => {
                input! {
                  pos: usize,
                  x: usize,
                }
                segtree.update(pos - 1, x);
            }
            2 => {
                input! {
                  l: usize,
                  r: usize,
                }
                println!("{}", segtree.query(l - 1, r - 1));
            }
            _ => unreachable!(),
        }
    }
}

struct SegmentTree {
    n: usize,
    node: Vec<usize>,
    monoid: fn(usize, usize) -> usize,
}

impl SegmentTree {
    fn new(n: usize, monoid: fn(usize, usize) -> usize) -> Self {
        Self {
            n,
            node: vec![0; n * 2],
            monoid,
        }
    }

    fn update(&mut self, pos: usize, x: usize) {
        let mut pos = pos + self.n - 1;
        self.node[pos] = x;
        while pos > 0 {
            pos /= 2;
            self.node[pos] = (self.monoid)(self.node[pos * 2], self.node[pos * 2 + 1]);
        }
    }

    fn query(&self, l: usize, r: usize) -> usize {
        let mut l = l + self.n - 1;
        let mut r = r + self.n - 1;
        let mut res = 0;
        while l < r {
            if l % 2 == 1 {
                res = (self.monoid)(res, self.node[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res = (self.monoid)(res, self.node[r]);
            }
            l /= 2;
            r /= 2;
        }
        res
    }

    fn get(&self, pos: usize) -> usize {
        self.node[pos + self.n - 1]
    }
}
