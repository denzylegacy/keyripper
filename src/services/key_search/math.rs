use num_bigint::BigUint;
use num_traits::One;
use k256::{AffinePoint, EncodedPoint, ProjectivePoint, Scalar};
use k256::elliptic_curve::group::GroupEncoding;

pub(crate) fn sqrt_mod_prime(y_square: &BigUint, p: &BigUint) -> Option<BigUint> {
    let exponent = (p + BigUint::one()) >> 2;
    let result = y_square.modpow(&exponent, p);

    if (&result * &result) % p == *y_square {
        Some(result)
    } else {
        None
    }
}

pub fn affine_coordinates(
    encoded_point: &EncodedPoint, target_public_key_point: ProjectivePoint, public_key_y: BigUint
) -> (BigUint, BigUint) {
    // println!("Encoded point {:?}", encoded_point);

    let affine_point = AffinePoint::from(target_public_key_point);

    let point_bytes = affine_point.to_bytes().as_slice();

    let affine_point = AffinePoint::from(target_public_key_point);

    // affine point to bytes
    let binding = affine_point.to_bytes();
    let point_bytes = binding.as_slice();

    // first byte is a prefix, the next 32 are x
    let x_bytes = &point_bytes[1..];

    let x_decimal = BigUint::from_bytes_be(x_bytes).to_str_radix(10);

    /// y

    let y_bytes = public_key_y.to_bytes_be();
    let y_decimal = BigUint::from_bytes_be(&y_bytes).to_str_radix(10);

    (x_decimal.parse().unwrap(), y_decimal.parse().unwrap())
}
