use k256::{ProjectivePoint, AffinePoint};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::ops::{AddAssign, Shr};
use k256::elliptic_curve::Group;
use k256::elliptic_curve::group::GroupEncoding;
use num_traits::{Zero, One, ToPrimitive};
use std::ops::Sub;

/// Baby-step Giant-step (BSGS) algorithm for solving discrete logarithm problem on elliptic curves.
/// This algorithm finds the scalar `k` such that `target_point = k * G`, where `G` is a generator point.
///
/// - `target_point` is the point we are trying to match.
/// - `g` is the generator point (usually `G`).
/// - `start` is the starting scalar value (offset).
/// - `max_steps` defines the range of search, i.e., the maximum number of steps for both baby and giant steps.
pub fn bsgs(
    target_point: &ProjectivePoint,
    g: &ProjectivePoint,
    start: &BigUint,
    max_steps: &BigUint,
) -> Option<BigUint> {
    println!("Start scalar: {:?}, Maximum steps: {:?}", start, max_steps);

    // Initialize a hash map to store baby steps with a capacity equal to `max_steps`.
    // This hash map will map affine coordinates (x, y) to their corresponding scalar `i`.
    let mut baby_steps = HashMap::with_capacity(
        max_steps.to_usize().unwrap_or(0)
    );

    // Calculate `current = g * start`, where `g` is the generator point and `start` is the scalar offset.
    let mut current = scalar_mul(g, start);

    // Baby-Step Phase:
    // // Baby-step phase: Compute all `g^i` for i in the range [0, max_steps], store in a hash map.
    let mut i = BigUint::zero();
    while &i < max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);

        baby_steps.insert((x_decimal, y_decimal), i.clone());

        current += g;

        i += BigUint::one();
    }

    // Giant-Step Phase:
    // Giant-step phase: Compute `target_point - j * g^m` for j in the range [0, max_steps], and check if it matches a baby step
    // This stride will be used to jump in larger steps during the search.
    let giant_stride = scalar_mul(g, max_steps);

    let mut current = target_point.clone();

    let mut j = BigUint::zero();

    while &j < max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);

        if let Some(i) = baby_steps.get(&(x_decimal.clone(), y_decimal.clone())) {
            // If match found, the result is `k = j * max_steps + i + start`
            let k = &j * max_steps + i + start;
            return Some(k);
        }

        current = current.sub(&giant_stride); // Move to the next giant step by subtracting `g^m`

        j += BigUint::one();
    }

    None // If no matching scalar `k` is found within the bounds, return `None`.
}

/// Scalar multiplication using the double-and-add algorithm.
/// Computes `result = scalar * point`, where `point` is a point on the elliptic curve
/// and `scalar` is a large integer (`k`).
/// Formula: result = k * P, where `P` is the elliptic curve point.
fn scalar_mul(point: &ProjectivePoint, scalar: &BigUint) -> ProjectivePoint {
    let mut result = ProjectivePoint::IDENTITY;
    let mut addend = *point;

    let mut scalar_bits = scalar.clone();

    while !scalar_bits.is_zero() && bool::from(!addend.is_identity()) {
        if &scalar_bits & BigUint::from(1u8) == BigUint::from(1u8) {
            result.add_assign(&addend);
        }

        addend = addend.double();

        scalar_bits = scalar_bits.shr(1);
    }

    result
}


/// Convert an elliptic curve point from affine coordinates to BigUint (x, y) coordinates.
/// This is necessary because elliptic curve points are typically stored in compressed form.
fn to_biguint_from_affine_point(point: &AffinePoint) -> (BigUint, BigUint) {
    let point_bytes = point.to_bytes();

    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..];

    let x_decimal = BigUint::from_bytes_be(x_bytes);
    let y_decimal = BigUint::from_bytes_be(y_bytes);

    (x_decimal, y_decimal)
}
