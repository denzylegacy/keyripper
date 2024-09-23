use k256::{ProjectivePoint, AffinePoint};
use num_bigint::BigUint;
use std::collections::HashMap;
use k256::elliptic_curve::group::GroupEncoding;
use num_traits::{Zero, One};

/// Scalar multiplication using the double-and-add algorithm.
/// Computes `result = scalar * point`, where `point` is a point on the elliptic curve
/// and `scalar` is a large integer (`k`).
/// Formula: result = k * P, where `P` is the elliptic curve point.
fn scalar_mul(point: &ProjectivePoint, scalar: &BigUint) -> ProjectivePoint {
    let mut result = ProjectivePoint::IDENTITY; // Identity element of the elliptic curve group
    let mut addend = *point;
    let mut k = scalar.clone(); // `k` is the scalar being multiplied

    // Double-and-add method to compute scalar multiplication
    while k > BigUint::zero() {
        // Add if the least significant bit of `k` is 1
        if &k & BigUint::one() == BigUint::one() {
            result += addend;
        }
        // Double the point (add point to itself)
        addend = addend.double();
        // Shift `k` one bit to the right (equivalent to dividing by 2)
        k >>= 1;
    }
    result
}

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
    println!("start: {:?}, max_steps: {:?}", start, max_steps);

    let mut baby_steps = HashMap::new();

    // Calculate `current = g * start`, where `g` is the generator point and `start` is the scalar offset.
    let mut current = scalar_mul(g, start);

    // Baby-step phase: Compute all `g^i` for i in the range [0, max_steps], store in a hash map.
    let mut i = BigUint::zero();
    while &i < max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);
        baby_steps.insert((x_decimal, y_decimal), i.clone());
        current += g; // Move to the next point in the sequence by adding `g`
        i += BigUint::one(); // Increment scalar
    }

    // Giant-step phase: Compute `target_point - j * g^m` for j in the range [0, max_steps], and check if it matches a baby step.
    let giant_stride = scalar_mul(g, max_steps); // Compute `g^m`, where `m = max_steps`
    let mut current = *target_point;

    let mut j = BigUint::zero();
    while &j < max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);

        // Check if the current giant step matches any of the stored baby steps
        if let Some(i) = baby_steps.get(&(x_decimal, y_decimal)) {
            // If match found, the result is `k = j * max_steps + i + start`
            let result = &j * max_steps + i + start;
            return Some(result);
        }
        current -= &giant_stride; // Move to the next giant step by subtracting `g^m`
        j += BigUint::one(); // Increment `j`
    }

    None // Return `None` if no match is found
}

/// Convert an elliptic curve point from affine coordinates to BigUint (x, y) coordinates.
/// This is necessary because elliptic curve points are typically stored in compressed form.
fn to_biguint_from_affine_point(point: &AffinePoint) -> (BigUint, BigUint) {
    let binding = point.to_bytes();
    let point_bytes = binding.as_slice();

    // The first byte is the prefix indicating the point format, the next 32 bytes are the x-coordinate,
    // and the remaining bytes represent the y-coordinate.
    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..];

    let x_decimal = BigUint::from_bytes_be(x_bytes); // Convert x-coordinate to BigUint
    let y_decimal = BigUint::from_bytes_be(y_bytes); // Convert y-coordinate to BigUint

    (x_decimal, y_decimal)
}
