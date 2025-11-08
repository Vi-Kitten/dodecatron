use std::{collections::HashSet, hash::Hash, ops::{Add, AddAssign, Mul, MulAssign, Neg, Not}};
use enumerable::Enumerable;

type Int = isize;
type Nat = usize;

trait CanMirror: Sized {
    fn flip(self) -> Self;
}

trait Group: Mul<Self, Output = Self> + Sized {
    fn neutral() -> Self;
    fn inverse(self) -> Self;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enumerable)]
struct Dihedral<G> {
    first_invert: bool,
    then_apply: G
}

impl<G> Dihedral<G> {
    fn map<H>(self, f: impl FnOnce(G) -> H) -> Dihedral<H> {
        Dihedral {
            first_invert: self.first_invert,
            then_apply: f(self.then_apply)
        }
    }
}

impl<G> From<G> for Dihedral<G> {
    fn from(value: G) -> Self {
        Dihedral {
            first_invert: false,
            then_apply: value
        }
    }
}

impl<G: Group> Mul<Dihedral<G>> for Dihedral<G> {
    type Output = Dihedral<G>;

    fn mul(self, rhs: Dihedral<G>) -> Self::Output {
        if self.first_invert {
            Dihedral {
                first_invert: rhs.first_invert.not(),
                then_apply: self.then_apply * rhs.then_apply.inverse()
            }
        } else {
            Dihedral {
                first_invert: rhs.first_invert,
                then_apply: self.then_apply * rhs.then_apply }
        }
    }
}

impl<G: Group> Group for Dihedral<G> {
    fn neutral() -> Self {
        G::neutral().into()
    }

    fn inverse(self) -> Self {
        if self.first_invert {
            self
        } else {
            self.then_apply.inverse().into()
        }
    }
}

impl<T: CanMirror, G: Mul<T, Output = T>> Mul<T> for Dihedral<G> {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        self.then_apply * if self.first_invert {
            rhs.flip()
        } else {
            rhs
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enumerable)]
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

type Dihedral6 = Dihedral<Cyclic6>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enumerable)]
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

type Dihedral12 = Dihedral<Cyclic12>;

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

fn reduce_by_symmetry<T: Clone + Eq + Hash, G: Enumerable + Group + Mul<T, Output = T>>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut sym_items = HashSet::<T>::new();
    iter.filter(move |t| {
        for x in G::enumerator() {
            if sym_items.contains(&(x * t.clone())) {
                return false
            }
        }
        sym_items.insert(t.clone());
        true
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Star6([Nat; 6]);

impl CanMirror for Star6 {
    fn flip(self: Self) -> Self {
        let Star6([x0, x1, x2, x3, x4, x5]) = self;
        Star6([x0, x5, x4, x3, x2, x1])
    }
}

impl Mul<Star6> for Cyclic6 {
    type Output = Star6;

    fn mul(self, Star6([x0, x1, x2, x3, x4, x5]): Star6) -> Self::Output {
        match self {
            Cyclic6::A0 => Star6([x0, x1, x2, x3, x4, x5]),
            Cyclic6::A1 => Star6([x1, x2, x3, x4, x5, x0]),
            Cyclic6::A2 => Star6([x2, x3, x4, x5, x0, x1]),
            Cyclic6::A3 => Star6([x3, x4, x5, x0, x1, x2]),
            Cyclic6::A4 => Star6([x4, x5, x0, x1, x2, x3]),
            Cyclic6::A5 => Star6([x5, x0, x1, x2, x3, x4]),
        }
    }
}

impl Star6 {
    /// Generate all combinations of directions that add to zero (ignoring symmetry)
    fn generate_raw_loops(n: Nat) -> Box<[Star6]> {
        // matrix from star to Z adjoin 6th root of unity
        // x  y  | u  | x' y' | u'
        // 1     | -1 | -1    | 1
        //    1  | 1  |    -1 | -1

        // dx = (x - x')
        // dy = (y - y')
        // uz = (u - u')

        // dx - du = 0
        // dy + du = 0
        // delta = dx = -dy = du

        // x_hat = min(x, x')
        // y_hat = min(y, y')
        // u_hat = min(u, u')

        // a = x_hat + y_hat + u_hat
        // b = abs(delta)

        // n = 2a + 3b

        let mut solutions = vec![];

        let mut a = (2 * n) % 3;

        loop {
            let Some(b) = n.checked_sub(2 * a).map(|m| m / 3) else {
                return solutions.into_boxed_slice();
            };
            for x_hat in 0..= a {
                for y_hat in 0..= (a - x_hat) {
                    let u_hat = a - x_hat - y_hat;
                    // delta = b
                    solutions.push(Star6([x_hat + b, y_hat, u_hat + b, x_hat, y_hat + b, u_hat]));
                    // delta = -b
                    solutions.push(Star6([x_hat, y_hat + b, u_hat, x_hat + b, y_hat, u_hat + b]));
                }
            }
            a += 3
        }
    }

    /// Generate all combinations of directions that add to zero
    fn generate_loops(n: Nat) -> Box<[Star6]> {
        reduce_by_symmetry::<Star6, Dihedral6>(Star6::generate_raw_loops(n).into_iter()).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Star12([Nat; 12]);

impl Star12 {
    fn from_interspersed(
        Star6([x0, x1, x2, x3, x4, x5]): Star6,
        Star6([y0, y1, y2, y3, y4, y5]): Star6
    ) -> Star12 {
        Star12([x0, y0, x1, y1, x2, y2, x3, y3, x4, y4, x5, y5])
    }
}

impl CanMirror for Star12 {
    fn flip(self) -> Self {
        let Star12([x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11]) = self;
        Star12([x0, x11, x10, x9, x8, x7, x6, x5, x4, x3, x2, x1])
    }
}

impl Mul<Star12> for Cyclic12 {
    type Output = Star12;

    fn mul(self, Star12([x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11]): Star12) -> Self::Output {
        match self {
            Cyclic12::A0 => Star12([x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11]),
            Cyclic12::A1 => Star12([x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x0]),
            Cyclic12::A2 => Star12([x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x0, x1]),
            Cyclic12::A3 => Star12([x3, x4, x5, x6, x7, x8, x9, x10, x11, x0, x1, x2]),
            Cyclic12::A4 => Star12([x4, x5, x6, x7, x8, x9, x10, x11, x0, x1, x2, x3]),
            Cyclic12::A5 => Star12([x5, x6, x7, x8, x9, x10, x11, x0, x1, x2, x3, x4]),
            Cyclic12::A6 => Star12([x6, x7, x8, x9, x10, x11, x0, x1, x2, x3, x4, x5]),
            Cyclic12::A7 => Star12([x7, x8, x9, x10, x11, x0, x1, x2, x3, x4, x5, x6]),
            Cyclic12::A8 => Star12([x8, x9, x10, x11, x0, x1, x2, x3, x4, x5, x6, x7]),
            Cyclic12::A9 => Star12([x9, x10, x11, x0, x1, x2, x3, x4, x5, x6, x7, x8]),
            Cyclic12::A10 => Star12([x10, x11, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9]),
            Cyclic12::A11 => Star12([x11, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10]),
        }
    }
}

impl Star12 {
    /// Generate all combinations of directions that add to zero
    fn generate_loops_upto(n: Nat) -> Box<[Box<[Star12]>]> {
        // matrix from star to Z adjoin 12th root of unity
        // x  y  z  w  | u' v' | x' y' z' w' | u' v'
        // 1           | -1    | -1          | 1
        //    1        |    -1 |    -1       |    1
        //       1     | 1     |       -1    | -1
        //          1  |    1  |          -1 |    -1

        // can be seperated into
        // x  y  | u  | x' y' | u'
        // 1     | -1 | -1    | 1
        //    1  | 1  |    -1 | -1
        // and
        // z  w  | v  | z' w' | v'
        // 1     | -1 | -1    | 1
        //    1  | 1  |    -1 | -1
        // by interspersing both the inputs and outputs

        let primary: Box<[Box<[Star6]>]> = (0..=n).map(Star6::generate_raw_loops).collect();
        let secondary: Box<[Box<[Star6]>]> = primary.iter().map(|stars|
            reduce_by_symmetry::<Star6, Dihedral6>(stars.iter().map(Star6::clone)).collect()
        ).collect();
        let mut bands: Box<[Vec<Star12>]> = (0..=n).map(|_| Vec::new()).collect();
        for n_primary in 0..=n {
            for n_secondary in 0..=(n - n_primary) {
                let m = n_primary + n_secondary;
                let primary_loops = &primary[n_primary];
                let secondary_loops = &secondary[n_secondary];
                primary_loops
                    .iter()
                    .map(|x| secondary_loops.iter().map(move |y| (x, y)))
                    .flatten()
                    .map(|(x, y)| Star12::from_interspersed(x.clone(), y.clone()))
                    .for_each(|star| {
                        bands[m].push(star)
                    });
            }
        }
        bands.into_iter().map(|band| reduce_by_symmetry::<Star12, Dihedral12>(band.into_iter()).collect()).collect()
    }
}

fn main() {
    let mut n = 0;
    for band in Star12::generate_loops_upto(6) {
        println!("Stars of size {}", n);
        for star in band {
            println!("{:?}", star)
        }
        n += 1
    }
}
