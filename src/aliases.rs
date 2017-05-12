/// Base-2 types.
pub mod binary {
    use typenum::U2;

    use Fix;

    pub type UFix8<Exp> = Fix<u8, U2, Exp>;
    pub type UFix16<Exp> = Fix<u16, U2, Exp>;
    pub type UFix32<Exp> = Fix<u32, U2, Exp>;
    pub type UFix64<Exp> = Fix<u64, U2, Exp>;
    pub type UFixSize<Exp> = Fix<usize, U2, Exp>;

    #[cfg(feature = "i128")]
    pub type UFix128<Exp> = Fix<u128, U2, Exp>;

    pub type IFix8<Exp> = Fix<i8, U2, Exp>;
    pub type IFix16<Exp> = Fix<i16, U2, Exp>;
    pub type IFix32<Exp> = Fix<i32, U2, Exp>;
    pub type IFix64<Exp> = Fix<i64, U2, Exp>;
    pub type IFixSize<Exp> = Fix<isize, U2, Exp>;

    #[cfg(feature = "i128")]
    pub type IFix128<Exp> = Fix<i128, U2, Exp>;
}

/// Base-10 types.
pub mod decimal {
    use typenum::U10;

    use Fix;

    pub type UFix8<Exp> = Fix<u8, U10, Exp>;
    pub type UFix16<Exp> = Fix<u16, U10, Exp>;
    pub type UFix32<Exp> = Fix<u32, U10, Exp>;
    pub type UFix64<Exp> = Fix<u64, U10, Exp>;
    pub type UFixSize<Exp> = Fix<usize, U10, Exp>;

    #[cfg(feature = "i128")]
    pub type UFix128<Exp> = Fix<u128, U10, Exp>;

    pub type IFix8<Exp> = Fix<i8, U10, Exp>;
    pub type IFix16<Exp> = Fix<i16, U10, Exp>;
    pub type IFix32<Exp> = Fix<i32, U10, Exp>;
    pub type IFix64<Exp> = Fix<i64, U10, Exp>;
    pub type IFixSize<Exp> = Fix<isize, U10, Exp>;

    #[cfg(feature = "i128")]
    pub type IFix128<Exp> = Fix<i128, U10, Exp>;
}

/// SI prefixes.
pub mod si {
    use typenum::{N1, N2, N3, N6, N9, N12, N15, N18, N21, N24};
    use typenum::{P1, P2, P3, P6, P9, P12, P15, P18, P21, P24};
    use typenum::{U10, Z0};

    use Fix;

    pub type Yocto<Bits> = Fix<Bits, U10, N24>;
    pub type Zepto<Bits> = Fix<Bits, U10, N21>;
    pub type Atto<Bits> = Fix<Bits, U10, N18>;
    pub type Femto<Bits> = Fix<Bits, U10, N15>;
    pub type Pico<Bits> = Fix<Bits, U10, N12>;
    pub type Nano<Bits> = Fix<Bits, U10, N9>;
    pub type Micro<Bits> = Fix<Bits, U10, N6>;
    pub type Milli<Bits> = Fix<Bits, U10, N3>;
    pub type Centi<Bits> = Fix<Bits, U10, N2>;
    pub type Deci<Bits> = Fix<Bits, U10, N1>;

    pub type Unit<Bits> = Fix<Bits, U10, Z0>;

    pub type Deca<Bits> = Fix<Bits, U10, P1>;
    pub type Hecto<Bits> = Fix<Bits, U10, P2>;
    pub type Kilo<Bits> = Fix<Bits, U10, P3>;
    pub type Mega<Bits> = Fix<Bits, U10, P6>;
    pub type Giga<Bits> = Fix<Bits, U10, P9>;
    pub type Tera<Bits> = Fix<Bits, U10, P12>;
    pub type Peta<Bits> = Fix<Bits, U10, P15>;
    pub type Exa<Bits> = Fix<Bits, U10, P18>;
    pub type Zeta<Bits> = Fix<Bits, U10, P21>;
    pub type Yotta<Bits> = Fix<Bits, U10, P24>;
}

/// IEC prefixes.
pub mod iec {
    use typenum::{P10, P20, P30, P40, P50, P60, P70, P80};
    use typenum::{U2, Z0};

    use Fix;

    pub type Unit<Bits> = Fix<Bits, U2, Z0>;

    pub type Kibi<Bits> = Fix<Bits, U2, P10>;
    pub type Mebi<Bits> = Fix<Bits, U2, P20>;
    pub type Gibi<Bits> = Fix<Bits, U2, P30>;
    pub type Tebi<Bits> = Fix<Bits, U2, P40>;
    pub type Pebi<Bits> = Fix<Bits, U2, P50>;
    pub type Exbi<Bits> = Fix<Bits, U2, P60>;
    pub type Zebi<Bits> = Fix<Bits, U2, P70>;
    pub type Yobi<Bits> = Fix<Bits, U2, P80>;
}
