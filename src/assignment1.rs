use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{Signed, Zero};

fn karatsuba_biguint(x: &BigUint, y: &BigUint) -> BigUint {
    if *x < BigUint::from(10u32) || *y < BigUint::from(10u32) {
        return x * y;
    }

    let n = x.to_str_radix(10).len().max(y.to_str_radix(10).len());
    let half_n = n / 2;

    let ten = BigUint::from(10u32);
    let ten_pow = ten.pow(half_n as u32);

    let a = x / &ten_pow;
    let b = x % &ten_pow;
    let c = y / &ten_pow;
    let d = y % &ten_pow;

    let a_c = karatsuba_biguint(&a, &c);
    let b_d = karatsuba_biguint(&b, &d);
    let ad_plus_bc = karatsuba_biguint(&(a.clone() + &b), &(c.clone() + &d)) - &a_c - &b_d;

    let ten_pow_sq = &ten_pow * &ten_pow;
    (ten_pow_sq * a_c) + (&ten_pow * ad_plus_bc) + b_d
}

/// Multiplies to integers using the Karatsuba method.
/// # Arguments
/// - integer: left multiplicand
/// - integer: right multiplicand
/// # Returns
/// The integer result of the multiplication of the multiplicands.
pub fn karatsuba(x: &BigInt, y: &BigInt) -> BigInt {
    if x.is_zero() || y.is_zero() {
        return BigInt::zero();
    }

    let sign = if x.sign() == y.sign() {
        Sign::Plus
    } else {
        Sign::Minus
    };

    let x_mag = x.abs().to_biguint().unwrap_or_else(BigUint::zero);
    let y_mag = y.abs().to_biguint().unwrap_or_else(BigUint::zero);

    let magnitude = karatsuba_biguint(&x_mag, &y_mag);
    BigInt::from_biguint(sign, magnitude)
}
