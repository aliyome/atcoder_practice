use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        x: isize,
        y: isize,
    };

    // (0,0) から +(1,2) または +(2,1) を繰り返して (x,y) に到達できるか？
    // -> 毎回座標3つずつ進むので、(x+y) % 3 == 0 であれば到達可能
    if (x + y) % 3 != 0 {
        println!("0");
        return;
    }

    // (x,y) に到達するには (x+y)/3 回の移動が必要
    let n = (x + y) / 3;

    // +(1,2), +(2,1) は扱いづらいので +(0,1), +(1,0) に変換したい。
    // そのために、(x,y) から n*(1,1) を引く
    let x = x - n;
    let y = y - n;

    // (x,y) が負の場合は到達不可能
    if x < 0 || y < 0 {
        println!("0");
        return;
    }

    let x = x as usize;
    let y = y as usize;

    // (0,0) から +(0,1), +(1,0) を繰り返して (x,y) に到達できるかという問題に帰着できる
    // -> (x,y) に到達するには x 回の +(0,1) と y 回の +(1,0) が必要
    // -> x+y C n 通りの組み合わせがあるのでこの計算結果が答え
    println!("{}", combination(x + y, x).value());
}

use std::ops;
#[derive(Copy, Clone)]
pub struct ModInt {
    value: usize,
}

impl ModInt {
    pub fn new(value: usize) -> ModInt {
        ModInt { value: value % MOD }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn inverse(&self) -> ModInt {
        // (a, b) -> (x, y) s.t. a * x + b * y = gcd(a, b)
        fn extended_gcd(a: isize, b: isize) -> (isize, isize) {
            if (a, b) == (1, 0) {
                (1, 0)
            } else {
                let (x, y) = extended_gcd(b, a % b);
                (y, x - (a / b) * y)
            }
        }

        let (x, _y) = extended_gcd(self.value() as isize, MOD as isize);
        ModInt::new((MOD as isize + x) as usize)
    }

    // a^n
    pub fn pow(&self, mut n: usize) -> ModInt {
        let mut res = ModInt::new(1);
        let mut x = *self;
        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }
            x *= x;
            n /= 2;
        }

        res
    }
}

impl ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.value + other.value)
    }
}

impl ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.value - other.value)
    }
}

impl ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: Self) -> Self {
        ModInt::new(self.value * other.value)
    }
}

impl ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl ops::DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// n!
pub fn factorial(n: usize) -> ModInt {
    (1..=n).fold(ModInt::new(1), |x, y| x * ModInt::new(y))
}

// nPr (https://ja.wikipedia.org/wiki/%E9%A0%86%E5%88%97#%E9%A0%86%E5%88%97%E3%81%AE%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92)
pub fn permutation(n: usize, r: usize) -> ModInt {
    if n < r {
        ModInt::new(0)
    } else {
        (n - r + 1..=n).fold(ModInt::new(1), |x, y| x * ModInt::new(y))
    }
}

// nCr (https://ja.wikipedia.org/wiki/%E7%B5%84%E5%90%88%E3%81%9B_(%E6%95%B0%E5%AD%A6))
pub fn combination(n: usize, r: usize) -> ModInt {
    if n < r {
        ModInt::new(0)
    } else {
        permutation(n, r) / factorial(r)
    }
}

// nHr (https://ja.wikipedia.org/wiki/%E9%87%8D%E8%A4%87%E7%B5%84%E5%90%88%E3%81%9B#%E9%87%8D%E8%A4%87%E7%B5%84%E5%90%88%E3%81%9B%E3%81%AE%E7%B7%8F%E6%95%B0)
pub fn homogeneous(n: usize, r: usize) -> ModInt {
    combination(n + r - 1, r)
}
