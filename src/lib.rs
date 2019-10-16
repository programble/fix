//! Fixed-point number types.
//!
//! # What?
//!
//! Fixed-point is a number representation with a fixed number of digits before and after the radix
//! point. This means that range is static rather than dynamic, as with floating-point. It also
//! means that they can be represented as integers, with their scale tracked by the type system.
//!
//! In this library, the scale of a `Fix` is represented as two type-level integers: the base and
//! the exponent. Any underlying integer primitive can be used to store the number. Arithmetic can
//! be performed on these numbers, and they can be converted to different scale exponents.
//!
//! # Why?
//!
//! A classic example: let's sum 10 cents and 20 cents using floating-point. We expect a result of
//! 30 cents.
//!
//! ```should_panic
//! assert_eq!(0.30, 0.10 + 0.20);
//! ```
//!
//! Wrong! We get an extra forty quintillionths of a dollar.
//!
//! ```text
//! assertion failed: `(left == right)` (left: `0.3`, right: `0.30000000000000004`)'
//! ```
//!
//! This is due to neither 0.1 nor 0.2 being exactly representable in base-2, just as a third can't
//! be represented exactly in base-10. With `Fix`, we can choose the precision we want in base-10,
//! at compile-time. In this case, hundredths of a dollar will do.
//!
//! ```
//! use fix::aliases::si::Centi; // Fix<_, U10, N2>
//! assert_eq!(Centi::new(0_30), Centi::new(0_10) + Centi::new(0_20));
//! ```
//!
//! But decimal is inefficient for binary computers, right? Multiplying and dividing by 10 is
//! slower than bit-shifting, but that's only needed when _moving_ the point. With `Fix`, this is
//! only done explicitly with the `convert` method.
//!
//! ```
//! use fix::aliases::si::{Centi, Milli};
//! assert_eq!(Milli::new(0_300), Centi::new(0_30).convert());
//! ```
//!
//! We can also choose a base-2 scale just as easily.
//!
//! ```
//! use fix::aliases::iec::{Kibi, Mebi};
//! assert_eq!(Kibi::new(1024), Mebi::new(1).convert());
//! ```
//!
//! It's also worth noting that the type-level scale changes when multiplying and dividing,
//! avoiding any implicit conversion.
//!
//! ```
//! use fix::aliases::iec::{Gibi, Kibi, Mebi};
//! assert_eq!(Mebi::new(3), Gibi::new(6) / Kibi::new(2));
//! ```
//!
//! # `no_std`
//!
//! This crate is `no_std`.
//!
//! # `i128` support
//!
//! Support for `u128` and `i128` can be enabled on nightly Rust through the `i128` Cargo feature.

#![no_std]

#![cfg_attr(feature = "i128", feature(i128_type))]

pub extern crate typenum;

/// Type aliases.
pub mod aliases;

use core::fmt::{Debug, Error, Formatter};
use core::marker::PhantomData;
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

use typenum::consts::Z0;
use typenum::marker_traits::{Bit, Integer, Unsigned};
use typenum::operator_aliases::{AbsVal, Diff, Le, Sum};
use typenum::type_operators::{Abs, IsLess};

/// Fixed-point number representing _Bits × Base <sup>Exp</sup>_.
///
/// - `Bits` is an integer primitive type, or any type which can be created from a type-level
///   integer and exponentiated.
/// - `Base` is an [`Unsigned`] type-level integer.
/// - `Exp` is a signed type-level [`Integer`].
///
/// [`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
/// [`Integer`]: ../typenum/marker_traits/trait.Integer.html
///
/// # Summary of operations
///
/// Lower case variables represent values of _Bits_. Upper case _B_ and _E_ represent type-level
/// integers _Base_ and _Exp_, respectively.
///
/// - _−(x B<sup>E</sup>) = (−x) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) + (y B<sup>E</sup>) = (x + y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) − (y B<sup>E</sup>) = (x − y) B<sup>E</sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) × (y B<sup>E<sub>y</sub></sup>) =
///   (x × y) B<sup>E<sub>x</sub> + E<sub>y</sub></sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) ÷ (y B<sup>E<sub>y</sub></sup>) =
///   (x ÷ y) B<sup>E<sub>x</sub> − E<sub>y</sub></sup>_
/// - _(x B<sup>E<sub>x</sub></sup>) % (y B<sup>E<sub>y</sub></sup>) =
///   (x % y) B<sup>E<sub>x</sub></sup>_
/// - _(x B<sup>E</sup>) × y = (x × y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) ÷ y = (x ÷ y) B<sup>E</sup>_
/// - _(x B<sup>E</sup>) % y = (x % y) B<sup>E</sup>_
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fix<Bits, Base, Exp> {
    /// The underlying integer.
    pub bits: Bits,

    marker: PhantomData<(Base, Exp)>,
}

impl<Bits, Base, Exp> Fix<Bits, Base, Exp> {
    /// Creates a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use fix::aliases::si::{Kilo, Milli};
    /// Milli::new(25); // 0.025
    /// Kilo::new(25); // 25 000
    /// ```
    pub fn new(bits: Bits) -> Self {
        Fix { bits, marker: PhantomData }
    }

    /// Converts to another _Exp_.
    ///
    /// # Examples
    ///
    /// ```
    /// use fix::aliases::si::{Kilo, Milli};
    /// let kilo = Kilo::new(5);
    /// let milli = Milli::new(5_000_000);
    /// assert_eq!(kilo, milli.convert());
    /// assert_eq!(milli, kilo.convert());
    /// ```
    pub fn convert<ToExp>(self) -> Fix<Bits, Base, ToExp>
    where
        Bits: FromUnsigned + Pow + Mul<Output = Bits> + Div<Output = Bits>,
        Base: Unsigned,
        Exp: Sub<ToExp>,
        Diff<Exp, ToExp>: Abs + IsLess<Z0>,
        AbsVal<Diff<Exp, ToExp>>: Integer
    {
        let base = Bits::from_unsigned::<Base>();
        let diff = AbsVal::<Diff<Exp, ToExp>>::to_i32();
        let inverse = Le::<Diff<Exp, ToExp>, Z0>::to_bool();

        // FIXME: Would like to do this with typenum::Pow, but that
        // seems to result in overflow evaluating requirements.
        let ratio = base.pow(diff as u32);

        if inverse {
            Fix::new(self.bits / ratio)
        } else {
            Fix::new(self.bits * ratio)
        }
    }
}

/// Conversion from type-level [`Unsigned`] integers.
///
/// Enables being generic over types which can be created from type-level integers. It should
/// probably be in `typenum` itself...
///
/// [`Unsigned`]: ../typenum/marker_traits/trait.Unsigned.html
pub trait FromUnsigned {
    /// Creates a value from a type.
    fn from_unsigned<U>() -> Self where U: Unsigned;
}

impl FromUnsigned for u8 { fn from_unsigned<U: Unsigned>() -> Self { U::to_u8() } }
impl FromUnsigned for u16 { fn from_unsigned<U: Unsigned>() -> Self { U::to_u16() } }
impl FromUnsigned for u32 { fn from_unsigned<U: Unsigned>() -> Self { U::to_u32() } }
impl FromUnsigned for u64 { fn from_unsigned<U: Unsigned>() -> Self { U::to_u64() } }
impl FromUnsigned for usize { fn from_unsigned<U: Unsigned>() -> Self { U::to_usize() } }

impl FromUnsigned for i8 { fn from_unsigned<U: Unsigned>() -> Self { U::to_i8() } }
impl FromUnsigned for i16 { fn from_unsigned<U: Unsigned>() -> Self { U::to_i16() } }
impl FromUnsigned for i32 { fn from_unsigned<U: Unsigned>() -> Self { U::to_i32() } }
impl FromUnsigned for i64 { fn from_unsigned<U: Unsigned>() -> Self { U::to_i64() } }
impl FromUnsigned for isize { fn from_unsigned<U: Unsigned>() -> Self { U::to_isize() } }

/// Exponentiation.
///
/// Enables being generic over integers which can be exponentiated. Why must we do this, standard
/// library?
pub trait Pow {
    /// Raises `self` to the power of `exp`.
    fn pow(self, exp: u32) -> Self;
}

impl Pow for u8 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for u16 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for u32 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for u64 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for usize { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }

impl Pow for i8 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for i16 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for i32 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for i64 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
impl Pow for isize { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }

#[cfg(feature = "i128")]
mod __i128 {
    use super::*;
    impl FromUnsigned for u128 { fn from_unsigned<U: Unsigned>() -> Self { U::to_u128() } }
    impl FromUnsigned for i128 { fn from_unsigned<U: Unsigned>() -> Self { U::to_i128() } }
    impl Pow for u128 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
    impl Pow for i128 { #[inline] fn pow(self, exp: u32) -> Self { self.pow(exp) } }
}

// The usual traits.

impl<Bits, Base, Exp> Debug for Fix<Bits, Base, Exp>
where Bits: Debug, Base: Unsigned, Exp: Integer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}x{}^{}", self.bits, Base::to_u64(), Exp::to_i64())
    }
}

// Arithmetic.

impl<Bits, Base, Exp> Neg for Fix<Bits, Base, Exp>
where Bits: Neg<Output = Bits> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.bits)
    }
}

impl<Bits, Base, Exp> Add for Fix<Bits, Base, Exp>
where Bits: Add<Output = Bits> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.bits + rhs.bits)
    }
}

impl<Bits, Base, Exp> Sub for Fix<Bits, Base, Exp>
where Bits: Sub<Output = Bits> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.bits - rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Mul<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: Mul<Output = Bits>, LExp: Add<RExp> {
    type Output = Fix<Bits, Base, Sum<LExp, RExp>>;
    fn mul(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.bits * rhs.bits)
    }
}

impl<Bits, Base, LExp, RExp> Div<Fix<Bits, Base, RExp>> for Fix<Bits, Base, LExp>
where Bits: Div<Output = Bits>, LExp: Sub<RExp> {
    type Output = Fix<Bits, Base, Diff<LExp, RExp>>;
    fn div(self, rhs: Fix<Bits, Base, RExp>) -> Self::Output {
        Self::Output::new(self.bits / rhs.bits)
    }
}

impl<Bits, Base, Exp> Rem for Fix<Bits, Base, Exp>
where Bits: Rem<Output = Bits> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(self.bits % rhs.bits)
    }
}

impl<Bits, Base, Exp> Mul<Bits> for Fix<Bits, Base, Exp>
where Bits: Mul<Output = Bits> {
    type Output = Self;
    fn mul(self, rhs: Bits) -> Self {
        Self::new(self.bits * rhs)
    }
}

impl<Bits, Base, Exp> Div<Bits> for Fix<Bits, Base, Exp>
where Bits: Div<Output = Bits> {
    type Output = Self;
    fn div(self, rhs: Bits) -> Self {
        Self::new(self.bits / rhs)
    }
}

impl<Bits, Base, Exp> Rem<Bits> for Fix<Bits, Base, Exp>
where Bits: Rem<Output = Bits> {
    type Output = Self;
    fn rem(self, rhs: Bits) -> Self {
        Self::new(self.bits % rhs)
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

#[cfg(test)]
mod tests {
    use aliases::si::{Kilo, Milli, Unit};

    #[test]
    fn convert_milli_to_kilo() {
        assert_eq!(Kilo::new(15), Milli::new(15_000_000).convert());
    }

    #[test]
    fn convert_kilo_to_milli() {
        assert_eq!(Milli::new(15_000_000), Kilo::new(15).convert());
    }

    #[test]
    fn cmp() {
        assert!(Kilo::new(1) < Kilo::new(2));
    }

    #[test]
    fn neg() {
        assert_eq!(Kilo::new(-1), -Kilo::new(1i32));
    }

    #[test]
    fn add() {
        assert_eq!(Kilo::new(3), Kilo::new(1) + Kilo::new(2));
    }

    #[test]
    fn sub() {
        assert_eq!(Kilo::new(1), Kilo::new(3) - Kilo::new(2));
    }

    #[test]
    fn mul() {
        assert_eq!(Unit::new(6), Kilo::new(2) * Milli::new(3));
    }

    #[test]
    fn div() {
        assert_eq!(Unit::new(3), Kilo::new(6) / Kilo::new(2));
    }

    #[test]
    fn rem() {
        assert_eq!(Kilo::new(1), Kilo::new(6) % Kilo::new(5));
    }

    #[test]
    fn mul_bits() {
        assert_eq!(Kilo::new(6), Kilo::new(2) * 3);
    }

    #[test]
    fn div_bits() {
        assert_eq!(Kilo::new(3), Kilo::new(6) / 2);
    }

    #[test]
    fn rem_bits() {
        assert_eq!(Kilo::new(1), Kilo::new(6) % 5);
    }

    #[test]
    fn add_assign() {
        let mut a = Kilo::new(1);
        a += Kilo::new(2);
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn sub_assign() {
        let mut a = Kilo::new(3);
        a -= Kilo::new(2);
        assert_eq!(Kilo::new(1), a);
    }

    #[test]
    fn mul_assign_bits() {
        let mut a = Kilo::new(2);
        a *= 3;
        assert_eq!(Kilo::new(6), a);
    }

    #[test]
    fn div_assign_bits() {
        let mut a = Kilo::new(6);
        a /= 2;
        assert_eq!(Kilo::new(3), a);
    }

    #[test]
    fn rem_assign() {
        let mut a = Kilo::new(6);
        a %= Milli::new(5);
        assert_eq!(Kilo::new(1), a);
    }

    #[test]
    fn rem_assign_bits() {
        let mut a = Kilo::new(6);
        a %= 5;
        assert_eq!(Kilo::new(1), a);
    }
}
