use k256::{ProjectivePoint, AffinePoint, Scalar};
use num_bigint::BigUint;
use std::collections::HashMap;
use k256::elliptic_curve::FieldBytes;
use k256::elliptic_curve::group::GroupEncoding;
use num_traits::ToPrimitive;

pub fn bsgs(
    target_point: &ProjectivePoint,
    g: &ProjectivePoint,
    start: &BigUint,
    max_steps: usize,
) -> Option<BigUint> {

    let mut baby_steps = HashMap::new();
    // let mut current = ProjectivePoint::IDENTITY;

    // BigUint -> u64 -> Scalar
    let start_scalar = Scalar::from(start.to_u64().expect("Conversion to u64 failed"));
    let mut current = g * &start_scalar;

    // BigUint -> u64 -> Scalar
    let max_steps_scalar_value = BigUint::from(max_steps as u64)
        .to_u64().expect("Conversion to u64 failed");
    let max_steps_scalar = Scalar::from(max_steps_scalar_value);

    // Baby-step
    for i in 0..max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);
        baby_steps.insert((x_decimal, y_decimal), i);
        current += *g; // + g in projective form
    }

    // Giant-step
    let giant_stride = g * &max_steps_scalar; // multiply in projective space
    let mut current = target_point.clone();

    for j in 0..max_steps {
        let affine_current = current.to_affine();
        let (x_decimal, y_decimal) = to_biguint_from_affine_point(&affine_current);
        if let Some(i) = baby_steps.get(&(x_decimal, y_decimal)) {
            return Some(BigUint::from(j as u64) * BigUint::from(max_steps as u64) + BigUint::from(*i as u64));
        }
        current -= giant_stride;
    }

    None
}

fn to_biguint_from_affine_point(point: &AffinePoint) -> (BigUint, BigUint) {
    let binding = point.to_bytes();
    let point_bytes = binding.as_slice();

    // First byte is a prefix, the next 32 are x, and the rest are y
    let x_bytes = &point_bytes[1..33];
    let y_bytes = &point_bytes[33..];

    let x_decimal = BigUint::from_bytes_be(x_bytes);
    let y_decimal = BigUint::from_bytes_be(y_bytes);

    (x_decimal, y_decimal)
}
