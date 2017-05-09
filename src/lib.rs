#![no_std]

pub extern crate typenum;

pub mod aliases;

use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use core::fmt::{Debug, Formatter, Error};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use typenum::{Unsigned, Integer, Sum, Diff};

pub struct Fix<Bits, Base, Exp> {
    bits: Bits,
    marker: PhantomData<(Base, Exp)>,
}

impl<Bits, Base, Exp> From<Bits> for Fix<Bits, Base, Exp> {
    fn from(bits: Bits) -> Self {
        Fix { bits, marker: PhantomData }
    }
}

impl<Bits, Base, Exp> Fix<Bits, Base, Exp> {
    pub fn into_bits(self) -> Bits {
        self.bits
    }
}

macro_rules! impl_convert {
    ($bits:ty, $to:ident, $to_i:ident) => {
        impl<Base, InExp> Fix<$bits, Base, InExp>
        where Base: Unsigned {
            pub fn convert<OutExp>(self) -> Fix<$bits, Base, OutExp>
            where InExp: Sub<OutExp>, Diff<InExp, OutExp>: Integer {
                let base = Base::$to();
                let diff = Diff::<InExp, OutExp>::$to_i();
                let ratio = base.pow(diff.abs() as u32);
                if diff < 0 {
                    Fix::from(self.bits / ratio)
                } else {
                    Fix::from(self.bits * ratio)
                }
            }
        }
    }
}

impl_convert!(u8, to_u8, to_i8);
impl_convert!(u16, to_u16, to_i16);
impl_convert!(u32, to_u32, to_i32);
impl_convert!(u64, to_u64, to_i64);
impl_convert!(usize, to_usize, to_isize);

impl_convert!(i8, to_i8, to_i8);
impl_convert!(i16, to_i16, to_i16);
impl_convert!(i32, to_i32, to_i32);
impl_convert!(i64, to_i64, to_i64);
impl_convert!(isize, to_isize, to_isize);

// The usual traits.

impl<Bits, Base, Exp> Copy for Fix<Bits, Base, Exp> where Bits: Copy { }
impl<Bits, Base, Exp> Clone for Fix<Bits, Base, Exp>
where Bits: Clone {
    fn clone(&self) -> Self {
        Self::from(self.bits.clone())
    }
}

impl<Bits, Base, Exp> Default for Fix<Bits, Base, Exp>
where Bits: Default {
    fn default() -> Self {
        Self::from(Bits::default())
    }
}

impl<Bits, Base, Exp> Hash for Fix<Bits, Base, Exp>
where Bits: Hash {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.bits.hash(state);
    }
}

impl<Bits, Base, Exp> Debug for Fix<Bits, Base, Exp>
where Bits: Debug, Base: Unsigned, Exp: Integer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}x{}^{}", self.bits, Base::to_u64(), Exp::to_i64())
    }
}

// Comparison.

impl<Bits, Base, Exp> Eq for Fix<Bits, Base, Exp> where Bits: Eq { }
impl<Bits, Base, Exp> PartialEq for Fix<Bits, Base, Exp>
where Bits: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.bits == rhs.bits
    }
}

impl<Bits, Base, Exp> PartialOrd for Fix<Bits, Base, Exp>
where Bits: PartialOrd {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.bits.partial_cmp(&rhs.bits)
    }
}

impl<Bits, Base, Exp> Ord for Fix<Bits, Base, Exp>
where Bits: Ord {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.bits.cmp(&rhs.bits)
    }
}

// Arithmetic.

impl<Bits, Base, Exp> Neg for Fix<Bits, Base, Exp>
where Bits: Neg<Output = Self> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from(-self.bits)
    }
}

impl<Bits, Base, Exp> Add for Fix<Bits, Base, Exp>
where Bits: Add<Output = Bits> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.bits + rhs.bits)
    }
}

impl<Bits, Base, Exp> Sub for Fix<Bits, Base, Exp>
where Bits: Sub<Output = Bits> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from(self.bits - rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Mul<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: Mul<Output = Bits>, LExp: Add<RExp> {
    type Output = Fix<Bits, Base, Sum<LExp, RExp>>;
    fn mul(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::from(self.bits * rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Div<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: Div<Output = Bits>, LExp: Sub<RExp> {
    type Output = Fix<Bits, Base, Diff<LExp, RExp>>;
    fn div(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::from(self.bits / rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Rem<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: Rem<Output = Bits> {
    type Output = Self;
    fn rem(self, rhs: Fix<Bits, Base, RExp>) -> Self {
        Self::from(self.bits % rhs.bits)
    }
}

impl<Bits, Base, Exp> Mul<Bits> for Fix<Bits, Base, Exp>
where Bits: Mul<Output = Bits> {
    type Output = Self;
    fn mul(self, rhs: Bits) -> Self {
        Self::from(self.bits * rhs)
    }
}

impl<Bits, Base, Exp> Div<Bits> for Fix<Bits, Base, Exp>
where Bits: Div<Output = Bits> {
    type Output = Self;
    fn div(self, rhs: Bits) -> Self {
        Self::from(self.bits / rhs)
    }
}

impl<Bits, Base, Exp> Rem<Bits> for Fix<Bits, Base, Exp>
where Bits: Rem<Output = Bits> {
    type Output = Self;
    fn rem(self, rhs: Bits) -> Self {
        Self::from(self.bits % rhs)
    }
}

// Assignment.

impl<Bits, Base, Exp> AddAssign for Fix<Bits, Base, Exp>
where Bits: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.bits += rhs.bits;
    }
}

impl<Bits, Base, Exp> SubAssign for Fix<Bits, Base, Exp>
where Bits: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.bits -= rhs.bits;
    }
}

impl<Bits, Base, Exp> MulAssign<Bits> for Fix<Bits, Base, Exp>
where Bits: MulAssign {
    fn mul_assign(&mut self, rhs: Bits) {
        self.bits *= rhs;
    }
}

impl<Bits, Base, Exp> DivAssign<Bits> for Fix<Bits, Base, Exp>
where Bits: DivAssign {
    fn div_assign(&mut self, rhs: Bits) {
        self.bits /= rhs;
    }
}

impl<Bits, Base, LExp, RExp> RemAssign<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: RemAssign {
    fn rem_assign(&mut self, rhs: Fix<Bits, Base, RExp>) {
        self.bits %= rhs.bits;
    }
}

impl<Bits, Base, Exp> RemAssign<Bits> for Fix<Bits, Base, Exp>
where Bits: RemAssign {
    fn rem_assign(&mut self, rhs: Bits) {
        self.bits %= rhs;
    }
}
