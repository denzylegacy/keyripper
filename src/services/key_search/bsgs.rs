use k256::{ProjectivePoint, AffinePoint};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::ops::{AddAssign, Sub};
use k256::elliptic_curve::Group;
use k256::elliptic_curve::group::GroupEncoding;
use num_traits::{Zero, One, ToPrimitive};
use std::ops::Shr;

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
    max_steps: u64,
) -> Option<BigUint> {
    println!("Start scalar: {:?}, Maximum steps: {}", start, max_steps);

    // Maps affine coordinates (x, y) to the index `i` (u64).
    let mut baby_steps: HashMap<(BigUint, BigUint), u64> = HashMap::with_capacity(max_steps as usize);

    // Calculate `current = g * start`
    let mut current = scalar_mul(g, start);

    // Baby-Step Phase: Compute all `g^i` for `i` in [0, max_steps)
    for i in 0..max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);

        baby_steps.insert((x_decimal, y_decimal), i);

        // Advance to the next point: `current += g`
        current += g;
    }

    // Giant-Step Phase:
    // Compute `giant_stride = g * max_steps`
    let giant_stride = scalar_mul(g, &BigUint::from(max_steps));

    // Initialize `current` with the target point
    let mut current = target_point.clone();

    // Iterate over `j` from 0 to `max_steps`
    for j in 0..max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);

        // Check if the current point is in the baby steps
        if let Some(&i) = baby_steps.get(&(x_decimal.clone(), y_decimal.clone())) {
            // Calculate `k = j * max_steps + i + start`
            let j_big = BigUint::from(j);
            let m_big = BigUint::from(max_steps);
            let k = &j_big * &m_big + BigUint::from(i) + start;

            return Some(k);
        }

        // Move to the next giant step: `current -= giant_stride`
        current = current.sub(&giant_stride);
    }

    None // If not found, return `None`
}

/// Scalar multiplication using the double-and-add algorithm.
/// Computes `result = scalar * point`.
fn scalar_mul(point: &ProjectivePoint, scalar: &BigUint) -> ProjectivePoint {
    let mut result = ProjectivePoint::IDENTITY;
    let mut addend = point.clone();

    let mut scalar_bits = scalar.clone();

    while !scalar_bits.is_zero() && bool::from(!addend.is_identity()) {
        if &scalar_bits & BigUint::one() == BigUint::one() {
            result.add_assign(&addend);
        }

        addend = addend.double();

        scalar_bits = scalar_bits.shr(1);
    }

    result
}

/// Convert an affine point to BigUint coordinates (x, y).
fn to_biguint_from_affine_point(point: &AffinePoint) -> (BigUint, BigUint) {
    let point_bytes = point.to_bytes();

    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..];

    let x_decimal = BigUint::from_bytes_be(x_bytes);
    let y_decimal = BigUint::from_bytes_be(y_bytes);

    (x_decimal, y_decimal)
}