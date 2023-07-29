use proconio::input;

fn main() {
    input! {
      w: usize,
    }

    if w == 1 {
        println!("12");
    } else {
        let m12 = ModInt::new(12);
        let m7 = ModInt::new(7);
        let ans = m12 * m7.pow(w - 1);
        println!("{}", ans.value());
    }
}

use std::ops;

const MOD: usize = 1000000007;

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
