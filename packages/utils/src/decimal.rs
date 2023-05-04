use rust_decimal::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes wasm compatible decimal wrapper.
#[derive(
    ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, Copy, Eq, PartialEq, Debug, Default,
)]
pub struct DecimalRatio {
    // numerator
    numerator: u128,
    // decimal number scale
    scale: u32,
}

impl DecimalRatio {
    /// ## Description
    /// Creates new instance of [`DecimalRatio`] with initial values
    /// ## Params
    /// * **numerator** is an object of type [`u128`]
    ///
    /// * **scale** is an object of type [`u32`]
    pub fn new(numerator: u128, scale: u32) -> Self {
        Self { numerator, scale }
    }

    /// ## Description
    /// Returns [`DecimalRatio`] with value equals to 0
    pub fn zero() -> Self {
        Decimal::ZERO.into()
    }

    /// ## Description
    /// Returns [`DecimalRatio`] with value equals to 1
    pub fn one() -> Self {
        Decimal::ONE.into()
    }

    /// ## Description
    /// Performes native decimal division and returns wrapped
    /// [`DecimalRatio`] result
    /// ## Params
    /// * **numerator** is an object of type [`u128`]
    ///
    /// * **denominator** is an object of type [`u129`]
    pub fn from_ratio(numerator: u128, denominator: u128) -> Self {
        let a = Decimal::from_u128(numerator).unwrap();
        let b = Decimal::from_u128(denominator).unwrap();

        a.checked_div(b).unwrap().into()
    }

    /// ## Description
    /// Returns [`u128`] converted value with cutted decimals
    pub fn to_u128(&self) -> u128 {
        Decimal::from_i128_with_scale(self.numerator as i128, self.scale)
            .to_u128()
            .unwrap()
    }
}

impl From<Decimal> for DecimalRatio {
    fn from(mut num: Decimal) -> Self {
        let scale = num.scale();
        num.set_scale(0).unwrap();

        Self {
            numerator: num.to_u128().unwrap(),
            scale,
        }
    }
}

impl From<DecimalRatio> for Decimal {
    fn from(dr: DecimalRatio) -> Self {
        Decimal::from_i128_with_scale(dr.numerator as i128, dr.scale)
    }
}

impl Add for DecimalRatio {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a: Decimal = self.into();
        let b: Decimal = rhs.into();

        a.checked_add(b).unwrap().into()
    }
}

impl Sub for DecimalRatio {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let a: Decimal = self.into();
        let b: Decimal = rhs.into();

        a.checked_sub(b).unwrap().into()
    }
}

impl Mul for DecimalRatio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a: Decimal = self.into();
        let b: Decimal = rhs.into();

        a.checked_mul(b).unwrap().into()
    }
}

impl Div for DecimalRatio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let a: Decimal = self.into();
        let b: Decimal = rhs.into();

        a.checked_div(b).unwrap().into()
    }
}

impl PartialOrd for DecimalRatio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DecimalRatio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a: Decimal = (*self).into();
        let b: Decimal = (*other).into();

        a.cmp(&b)
    }
}

impl ToString for DecimalRatio {
    fn to_string(&self) -> String {
        let d: Decimal = (*self).into();
        d.to_string()
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::*;

    use super::*;

    #[test]
    fn test_decimal_math() {
        let a = DecimalRatio::new(100, 3);
        let b = DecimalRatio::new(400, 0);

        let res: Decimal = (a / b).into();
        assert_eq!(res.to_string(), "0.00025");

        let dr_res: DecimalRatio = res.into();
        assert_eq!(dr_res, DecimalRatio::new(25, 5));
        assert_eq!(dr_res.to_u128(), 0);

        let ratio = DecimalRatio::from_ratio(1678, 909841);
        assert_eq!(ratio, DecimalRatio::new(18442782859862327593502601, 28));
        assert_eq!(ratio.to_u128(), 0);
        let dec_ratio: Decimal = ratio.into();
        assert_eq!(dec_ratio.to_string(), "0.0018442782859862327593502601");

        let res = (DecimalRatio::new(123, 5) + DecimalRatio::new(4444, 2))
            * (DecimalRatio::new(556436, 3) - DecimalRatio::new(431, 0))
            / DecimalRatio::new(489314, 4);
        assert_eq!(res, DecimalRatio::new(11392541652762847578446559878, 26));
    }

    #[test]
    fn test_cmp() {
        let a = DecimalRatio::new(100, 3);
        let b = DecimalRatio::new(400, 0);

        assert_eq!(b > a && b >= a, true);
        assert_eq!(a < b && a <= b, true);

        let eq1 = DecimalRatio::new(100, 3);
        let eq2 = DecimalRatio::new(100, 3);
        assert_eq!(eq1 == eq2, true);
    }
}
