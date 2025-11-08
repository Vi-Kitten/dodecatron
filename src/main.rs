use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

type Int = isize;
type Nat = usize;

trait Group: Mul<Self, Output = Self> + Sized {
    fn neutral() -> Self;
    fn inverse(self) -> Self;
}

struct Dihedral<G> {
    first_invert: bool,
    then_apply: G
}

#[derive(Debug, PartialEq, Eq)]
enum Cyclic6 {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
}

impl Cyclic6 {
    fn as_c12(self) -> Cyclic12 {
        match self {
            Cyclic6::A0 => Cyclic12::A0,
            Cyclic6::A1 => Cyclic12::A2,
            Cyclic6::A2 => Cyclic12::A4,
            Cyclic6::A3 => Cyclic12::A6,
            Cyclic6::A4 => Cyclic12::A8,
            Cyclic6::A5 => Cyclic12::A10,
        }
    }
}

impl Mul for Cyclic6 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Cyclic6::*;
        match (self, rhs) {
            (A0, A0) => A0,
            (A0, A1) => A1,
            (A0, A2) => A2,
            (A0, A3) => A3,
            (A0, A4) => A4,
            (A0, A5) => A5,

            (A1, A0) => A1,
            (A1, A1) => A2,
            (A1, A2) => A3,
            (A1, A3) => A4,
            (A1, A4) => A5,
            (A1, A5) => A0,

            (A2, A0) => A2,
            (A2, A1) => A3,
            (A2, A2) => A4,
            (A2, A3) => A5,
            (A2, A4) => A0,
            (A2, A5) => A1,

            (A3, A0) => A3,
            (A3, A1) => A4,
            (A3, A2) => A5,
            (A3, A3) => A0,
            (A3, A4) => A1,
            (A3, A5) => A2,

            (A4, A0) => A4,
            (A4, A1) => A5,
            (A4, A2) => A0,
            (A4, A3) => A1,
            (A4, A4) => A2,
            (A4, A5) => A3,

            (A5, A0) => A5,
            (A5, A1) => A0,
            (A5, A2) => A1,
            (A5, A3) => A2,
            (A5, A4) => A3,
            (A5, A5) => A4,
        }
    }
}

impl Group for Cyclic6 {
    fn neutral() -> Self {
        Cyclic6::A0
    }

    fn inverse(self) -> Self {
        use Cyclic6::*;
        match self {
            A0 => A0,
            A1 => A5,
            A2 => A4,
            A3 => A3,
            A4 => A2,
            A5 => A1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dihedral6 {
    first_flip: bool,
    then_rotate: Cyclic6,
}

impl Dihedral6 {
    fn as_d12(self) -> Dihedral12 {
        Dihedral12 {
            first_flip: self.first_flip,
            then_rotate: self.then_rotate.as_c12(),
        }
    }
}

impl From<Cyclic6> for Dihedral6 {
    fn from(value: Cyclic6) -> Self {
        Dihedral6 {
            first_flip: false,
            then_rotate: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cyclic12 {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
}

impl Mul for Cyclic12 {
    type Output = Cyclic12;

    fn mul(self, rhs: Self) -> Self::Output {
        use Cyclic12::*;
        match (self, rhs) {
            (A0, A0) => A0,
            (A0, A1) => A1,
            (A0, A2) => A2,
            (A0, A3) => A3,
            (A0, A4) => A4,
            (A0, A5) => A5,
            (A0, A6) => A6,
            (A0, A7) => A7,
            (A0, A8) => A8,
            (A0, A9) => A9,
            (A0, A10) => A10,
            (A0, A11) => A11,

            (A1, A0) => A1,
            (A1, A1) => A2,
            (A1, A2) => A3,
            (A1, A3) => A4,
            (A1, A4) => A5,
            (A1, A5) => A6,
            (A1, A6) => A7,
            (A1, A7) => A8,
            (A1, A8) => A9,
            (A1, A9) => A10,
            (A1, A10) => A11,
            (A1, A11) => A0,

            (A2, A0) => A2,
            (A2, A1) => A3,
            (A2, A2) => A4,
            (A2, A3) => A5,
            (A2, A4) => A6,
            (A2, A5) => A7,
            (A2, A6) => A8,
            (A2, A7) => A9,
            (A2, A8) => A10,
            (A2, A9) => A11,
            (A2, A10) => A0,
            (A2, A11) => A1,

            (A3, A0) => A3,
            (A3, A1) => A4,
            (A3, A2) => A5,
            (A3, A3) => A6,
            (A3, A4) => A7,
            (A3, A5) => A8,
            (A3, A6) => A9,
            (A3, A7) => A10,
            (A3, A8) => A11,
            (A3, A9) => A0,
            (A3, A10) => A1,
            (A3, A11) => A2,

            (A4, A0) => A4,
            (A4, A1) => A5,
            (A4, A2) => A6,
            (A4, A3) => A7,
            (A4, A4) => A8,
            (A4, A5) => A9,
            (A4, A6) => A10,
            (A4, A7) => A11,
            (A4, A8) => A0,
            (A4, A9) => A1,
            (A4, A10) => A2,
            (A4, A11) => A3,

            (A5, A0) => A5,
            (A5, A1) => A6,
            (A5, A2) => A7,
            (A5, A3) => A8,
            (A5, A4) => A9,
            (A5, A5) => A10,
            (A5, A6) => A11,
            (A5, A7) => A0,
            (A5, A8) => A1,
            (A5, A9) => A2,
            (A5, A10) => A3,
            (A5, A11) => A4,

            (A6, A0) => A6,
            (A6, A1) => A7,
            (A6, A2) => A8,
            (A6, A3) => A9,
            (A6, A4) => A10,
            (A6, A5) => A11,
            (A6, A6) => A0,
            (A6, A7) => A1,
            (A6, A8) => A2,
            (A6, A9) => A3,
            (A6, A10) => A4,
            (A6, A11) => A5,

            (A7, A0) => A7,
            (A7, A1) => A8,
            (A7, A2) => A9,
            (A7, A3) => A10,
            (A7, A4) => A11,
            (A7, A5) => A0,
            (A7, A6) => A1,
            (A7, A7) => A2,
            (A7, A8) => A3,
            (A7, A9) => A4,
            (A7, A10) => A5,
            (A7, A11) => A6,

            (A8, A0) => A8,
            (A8, A1) => A9,
            (A8, A2) => A10,
            (A8, A3) => A11,
            (A8, A4) => A0,
            (A8, A5) => A1,
            (A8, A6) => A2,
            (A8, A7) => A3,
            (A8, A8) => A4,
            (A8, A9) => A5,
            (A8, A10) => A6,
            (A8, A11) => A7,

            (A9, A0) => A9,
            (A9, A1) => A10,
            (A9, A2) => A11,
            (A9, A3) => A0,
            (A9, A4) => A1,
            (A9, A5) => A2,
            (A9, A6) => A3,
            (A9, A7) => A4,
            (A9, A8) => A5,
            (A9, A9) => A6,
            (A9, A10) => A7,
            (A9, A11) => A8,

            (A10, A0) => A10,
            (A10, A1) => A11,
            (A10, A2) => A0,
            (A10, A3) => A1,
            (A10, A4) => A2,
            (A10, A5) => A3,
            (A10, A6) => A4,
            (A10, A7) => A5,
            (A10, A8) => A6,
            (A10, A9) => A7,
            (A10, A10) => A8,
            (A10, A11) => A9,

            (A11, A0) => A11,
            (A11, A1) => A0,
            (A11, A2) => A1,
            (A11, A3) => A2,
            (A11, A4) => A3,
            (A11, A5) => A4,
            (A11, A6) => A5,
            (A11, A7) => A6,
            (A11, A8) => A7,
            (A11, A9) => A8,
            (A11, A10) => A9,
            (A11, A11) => A10,
        }
    }
}

impl Group for Cyclic12 {
    fn neutral() -> Self {
        Cyclic12::A0
    }

    fn inverse(self) -> Self {
        use Cyclic12::*;
        match self {
            A0 => A0,
            A1 => A11,
            A2 => A10,
            A3 => A9,
            A4 => A8,
            A5 => A7,
            A6 => A6,
            A7 => A5,
            A8 => A4,
            A9 => A3,
            A10 => A2,
            A11 => A1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dihedral12 {
    first_flip: bool,
    then_rotate: Cyclic12,
}

impl From<Cyclic12> for Dihedral12 {
    fn from(value: Cyclic12) -> Self {
        Dihedral12 {
            first_flip: false,
            then_rotate: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Poly12Step {
    AntiClock5,
    AntiClock4,
    AntiClock3,
    AntiClock2,
    AntiClock1,
    Clock1,
    Clock2,
    Clock3,
    Clock4,
    Clock5,
}

#[derive(Debug)]
struct Path12 {
    steps: Vec<Poly12Step>
}

#[derive(Debug, PartialEq, Eq)]
struct Cyclotomic12 {
    z0: Int,
    z1: Int,
    z2: Int,
    z3: Int
}

impl Cyclotomic12 {
    fn zero() -> Cyclotomic12 {
        Cyclotomic12 { z0: 0, z1: 0, z2: 0, z3: 0 }
    }

    fn unit() -> Cyclotomic12 {
        Cyclotomic12 { z0: 1, z1: 0, z2: 0, z3: 0 }
    }

    fn zeta() -> Cyclotomic12 {
        Cyclotomic12 { z0: 1, z1: 0, z2: 0, z3: 0 }
    }

    fn as_f64_coordinate(self) -> (f64, f64) {
        // can't be const because rust hates me
        let _sqrt_3_over_2 = 3_f64.sqrt() * 0.5_f64;
        let cos_30 = _sqrt_3_over_2;
        let sin_30 = 0.5_f64;
        let cos_60 = 0.5_f64;
        let sin_60 = _sqrt_3_over_2;
        
        let mut x = 0_f64;
        let mut y = 0_f64;

        x += self.z0 as f64;

        x += self.z1 as f64 * cos_30;
        y += self.z1 as f64 * sin_30;

        x += self.z2 as f64 * cos_60;
        y += self.z2 as f64 * sin_60;

        y += self.z3 as f64;

        (x, y)
    }
}

impl From<Poly12Step> for Cyclotomic12 {
    fn from(value: Poly12Step) -> Self {
        match value {
            Poly12Step::AntiClock5 => Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::AntiClock4 => Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::AntiClock3 => Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::AntiClock2 => Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::AntiClock1 => Cyclotomic12::zeta(),
            Poly12Step::Clock1 => -Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::Clock2 => -Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::Clock3 => -Cyclotomic12::zeta() * Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::Clock4 => -Cyclotomic12::zeta() * Cyclotomic12::zeta(),
            Poly12Step::Clock5 => -Cyclotomic12::zeta(),
        }
    }
}

impl Add for Cyclotomic12 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cyclotomic12 {
            z0: self.z0 + rhs.z0,
            z1: self.z1 + rhs.z1,
            z2: self.z2 + rhs.z2,
            z3: self.z3 + rhs.z3,
        }
    }
}

impl Neg for Cyclotomic12 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Cyclotomic12 {
            z0: -self.z0,
            z1: -self.z1,
            z2: -self.z2,
            z3: -self.z3,
        }
    }
}

impl Mul for Cyclotomic12 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut z = Cyclotomic12::zero();

        z.z0 += self.z0 * rhs.z0;
        z.z1 += self.z0 * rhs.z1;
        z.z2 += self.z0 * rhs.z2;
        z.z3 += self.z0 * rhs.z3;

        z.z1 += self.z1 * rhs.z0;
        z.z2 += self.z1 * rhs.z1;
        z.z3 += self.z1 * rhs.z2;
        z.z0 -= self.z1 * rhs.z3;
        z.z2 += self.z1 * rhs.z3;

        z.z2 += self.z2 * rhs.z0;
        z.z3 += self.z2 * rhs.z1;
        z.z0 -= self.z2 * rhs.z2;
        z.z2 += self.z2 * rhs.z2;
        z.z1 -= self.z2 * rhs.z3;
        z.z3 += self.z2 * rhs.z3;

        z.z3 += self.z3 * rhs.z0;
        z.z0 -= self.z3 * rhs.z1;
        z.z2 += self.z3 * rhs.z1;
        z.z1 -= self.z3 * rhs.z2;
        z.z3 += self.z3 * rhs.z2;
        z.z0 -= self.z3 * rhs.z3;

        z
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Star6([Nat; 6]);

impl Star6 {
    /// Generate all combinations of directions that add to zero (ignoring symmetry)
    fn generate_raw_loops(n: Nat) -> Vec<Star6> {
        // x  y  | z  | x' y' | z'
        // 1     | -1 | -1    | 1
        //    1  | 1  |    -1 | -1

        // dx = (x - x')
        // dy = (y - y')
        // dz = (z - z')

        // dx - dz = 0
        // dy + dz = 0
        // delta = dx = -dy = dz

        // x_hat = min(x, x')
        // y_hat = min(y, y')
        // z_hat = min(z, z')

        // a = x_hat + y_hat + z_hat
        // b = abs(delta)

        // n = 2a + 3b

        let mut solutions = vec![];

        let mut a = (2 * n) % 3;

        loop {
            let Some(b) = n.checked_sub(2 * 3).map(|m| m / 3) else {
                return solutions;
            };
            for x_hat in 0..= a {
                for y_hat in 0..= (a - x_hat) {
                    let z_hat = a - x_hat - y_hat;
                    // delta = b
                    solutions.push(Star6([x_hat + b, y_hat, z_hat + b, x_hat, y_hat + b, z_hat]));
                    // delta = -b
                    solutions.push(Star6([x_hat, y_hat + b, z_hat, x_hat + b, y_hat, z_hat + b]));
                }
            }
            a += 3
        }
    }
}

fn main() {
    println!("Hello, world!");
}
