use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lr: [(usize, usize); n]
    }

    // // 全探索 O(NW) TLE
    // let mut brick = vec![0; w + 1];
    // for i in 0..n {
    //     let (l, r) = lr[i];
    //     let mut max = 0;
    //     for j in l..=r {
    //         max = max.max(brick[j]);
    //     }
    //     for j in l..=r {
    //         brick[j] = max + 1;
    //     }
    //     println!("{}", max + 1);
    // }

    // // 座標圧縮 O(N + N^2)
    // let mut set = HashSet::new();
    // for i in 0..n {
    //     let (l, r) = lr[i];
    //     set.insert(l);
    //     set.insert(r);
    // }
    // let mut rank = HashMap::new();
    // for (i, v) in set.iter().sorted().enumerate() {
    //     rank.insert(v, i);
    // }
    // let mut brick = vec![0; n * 2 + 1];
    // for i in 0..n {
    //     let (l, r) = lr[i];
    //     let l = *rank.get(&l).unwrap();
    //     let r = *rank.get(&r).unwrap();
    //     let mut max = 0;
    //     for j in l..=r {
    //         max = max.max(brick[j]);
    //     }
    //     for j in l..=r {
    //         brick[j] = max + 1;
    //     }
    //     println!("{}", max + 1);
    // }

    // TODO: 解答ACなので後で復習
    // 遅延評価セグメントツリー O(W + NlogW)
    let op = |x: usize, y: usize| x.max(y);
    let element = || 0usize;
    let mapping = |l: Option<usize>, v: usize| match l {
        Some(val) => val,
        None => v,
    };
    let id = || None::<usize>;
    let composite = |f: Option<usize>, g: Option<usize>| match f {
        Some(_) => f,
        None => g,
    };
    let mut tree = LazySegmentTree::new(w + 1, op, element, mapping, id, composite);
    for i in 0..n {
        let (l, r) = lr[i];
        let next_height = tree.prod(l - 1, r) + 1;
        println!("{}", next_height);
        tree.apply(Some(next_height), l - 1, r);
    }
}

// 引用:
// [Rustで遅延評価セグメントツリーを実装してAtCoder Typical90の029を解く。 - Qiita](https://qiita.com/Yosemat1/items/66a93c1ca4358245c75a)
pub struct LazySegmentTree<S, F, E, T, G, H, I>
where
    S: Copy + Eq,
    T: Copy + Eq,
    F: Fn(S, S) -> S,
    E: Fn() -> S,
    G: Fn(T, S) -> S,
    H: Fn() -> T,
    I: Fn(T, T) -> T,
{
    value: Vec<S>,
    op: F,
    element: E,
    lazy: Vec<T>,
    mapping: G,
    id: H,
    composite: I,
}

impl<S, F, E, T, G, H, I> LazySegmentTree<S, F, E, T, G, H, I>
where
    S: Copy + Eq,
    T: Copy + Eq,
    F: Fn(S, S) -> S,
    E: Fn() -> S,
    G: Fn(T, S) -> S,
    H: Fn() -> T,
    I: Fn(T, T) -> T,
{
    pub fn new(size: usize, op: F, element: E, mapping: G, id: H, composite: I) -> Self {
        let tree_size = size.next_power_of_two() * 2 - 1;
        let value = vec![element(); tree_size];
        let lazy = vec![id(); tree_size];
        Self {
            value,
            op,
            element,
            lazy,
            mapping,
            id,
            composite,
        }
    }

    pub fn prod(&mut self, left: usize, right: usize) -> S {
        assert!(left <= right);
        if right == left {
            (self.element)()
        } else {
            self._prod(0, left, right, 0, self.value.len() / 2 + 1)
        }
    }

    fn _prod(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) -> S {
        if search_left <= left && right <= search_right {
            self.value[tree_index]
        } else if right <= search_left || search_right <= left {
            (self.element)()
        } else {
            if self.lazy[tree_index] != (self.id)() {
                self.propagate(tree_index, left, right, left, right);
            }
            let mid = (left + right) / 2;
            let (left_t_index, right_t_index) = self.get_children(tree_index);
            let l_value = self._prod(left_t_index, search_left, search_right, left, mid);
            let r_value = self._prod(right_t_index, search_left, search_right, mid, right);
            (self.op)(l_value, r_value)
        }
    }

    fn get_children(&self, tree_index: usize) -> (usize, usize) {
        (tree_index * 2 + 1, tree_index * 2 + 2)
    }

    pub fn apply(&mut self, v: T, left: usize, right: usize) {
        self._apply(v, 0, left, right, 0, self.value.len() / 2 + 1);
    }

    fn _apply(
        &mut self,
        v: T,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        if right <= search_left || search_right <= left {
            return;
        }
        self.lazy[tree_index] = (self.composite)(v, self.lazy[tree_index]);
        if search_left <= left && right <= search_right {
            self.value[tree_index] = (self.mapping)(v, self.value[tree_index]);
        } else {
            self.propagate(tree_index, search_left, search_right, left, right);
            let (left_t_index, right_t_index) = self.get_children(tree_index);
            self.value[tree_index] = (self.op)(self.value[left_t_index], self.value[right_t_index]);
        }
    }

    fn propagate(
        &mut self,
        tree_index: usize,
        search_left: usize,
        search_right: usize,
        left: usize,
        right: usize,
    ) {
        let lazy = self.lazy[tree_index];
        self.lazy[tree_index] = (self.id)();
        let mid = (left + right) / 2;
        let (left_t_index, right_t_index) = self.get_children(tree_index);
        self._apply(lazy, left_t_index, search_left, search_right, left, mid);
        self._apply(lazy, right_t_index, search_left, search_right, mid, right);
    }
}
