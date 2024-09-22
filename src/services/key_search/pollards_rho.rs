/*use k256::ProjectivePoint;
use num_bigint::BigUint;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
use num_traits::{Zero, One, ToPrimitive};
use std::ops::{Add, Sub, Mul, Rem};
use rand_bigint::RandBigInt;

fn get_group_order() -> BigUint {
    // The group order of the secp256k1 curve used in k256
    let group_order_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
    BigUint::parse_bytes(group_order_hex.as_bytes(), 16).unwrap()
}

fn point_mul(point: &ProjectivePoint, scalar: &BigUint) -> ProjectivePoint {
    let scalar_bytes = scalar.to_bytes_be();

    // Ensure scalar_bytes is 32 bytes
    let mut scalar_bytes_padded = [0u8; 32];
    let scalar_bytes_len = scalar_bytes.len();
    if scalar_bytes_len > 32 {
        panic!("Scalar too large");
    }
    scalar_bytes_padded[32 - scalar_bytes_len..].copy_from_slice(&scalar_bytes);

    // Use the multiply method that accepts a byte array
    point.multiply(&scalar_bytes_padded)
}

fn modinv(a: &BigUint, modulus: &BigUint) -> Option<BigUint> {
    // Extended Euclidean Algorithm
    let mut mn = (modulus.clone(), a.clone());
    let mut xy = (BigUint::zero(), BigUint::one());

    while mn.1 != BigUint::zero() {
        let q = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &q * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &q * &xy.1);
    }

    if mn.0 == BigUint::one() {
        Some((xy.0 + modulus) % modulus)
    } else {
        None
    }
}

fn sub_mod(a: &BigUint, b: &BigUint, modulus: &BigUint) -> BigUint {
    if a >= b {
        (a - b) % modulus
    } else {
        (a + modulus - b) % modulus
    }
}

pub fn pollards_rho(
    target_point: &ProjectivePoint,
    g: &ProjectivePoint,
    num_threads: usize,
) -> Option<BigUint> {
    let modulus = get_group_order();

    let found_key = Arc::new(Mutex::new(None));
    let threads: Vec<_> = (0..num_threads)
        .map(|_| {
            let target_point = *target_point;
            let g = *g;
            let found_key = Arc::clone(&found_key);

            thread::spawn(move || {
                let mut rng = thread_rng();

                // Generate random a, b in [0, n-1]
                let a = rng.gen_biguint_below(&modulus);
                let b = rng.gen_biguint_below(&modulus);

                let mut x = point_mul(&g, &a) + point_mul(&target_point, &b);
                let mut a1 = a.clone();
                let mut b1 = b.clone();

                let mut x2 = x;
                let mut a2 = a1.clone();
                let mut b2 = b1.clone();

                loop {
                    // Check if the key has been found by another thread
                    if found_key.lock().unwrap().is_some() {
                        break;
                    }

                    // Single step
                    let (new_x, new_a1, new_b1) =
                        update_point(&x, a1.clone(), b1.clone(), &g, &target_point, &modulus);
                    x = new_x;
                    a1 = new_a1;
                    b1 = new_b1;

                    // Double step
                    for _ in 0..2 {
                        let (new_x2, new_a2, new_b2) =
                            update_point(&x2, a2.clone(), b2.clone(), &g, &target_point, &modulus);
                        x2 = new_x2;
                        a2 = new_a2;
                        b2 = new_b2;
                    }

                    if x == x2 {
                        let numerator = sub_mod(&a1, &a2, &modulus);
                        let denominator = sub_mod(&b2, &b1, &modulus);

                        if denominator.is_zero() {
                            // Denominator is zero; restart with new random values
                            break;
                        }

                        if let Some(inv_denominator) = modinv(&denominator, &modulus) {
                            let k = (&numerator * &inv_denominator) % &modulus;

                            // Verify the solution
                            if point_mul(&g, &k) == *target_point {
                                let mut found = found_key.lock().unwrap();
                                *found = Some(k);
                                break;
                            }
                        } else {
                            // Inverse doesn't exist; restart with new random values
                            break;
                        }
                    }
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    Arc::try_unwrap(found_key).ok().unwrap().lock().unwrap().clone()
}

fn update_point(
    point: &ProjectivePoint,
    a: BigUint,
    b: BigUint,
    g: &ProjectivePoint,
    target_point: &ProjectivePoint,
    modulus: &BigUint,
) -> (ProjectivePoint, BigUint, BigUint) {
    let x_coord = point.to_affine().x().unwrap().to_bytes();
    let x_int = BigUint::from_bytes_be(&x_coord);

    match (x_int.clone() % BigUint::from(3u8)).to_u8().unwrap() {
        0 => {
            // Point = 2 * Point
            let new_point = point.double();
            let new_a = (a.clone() + a) % modulus;
            let new_b = (b.clone() + b) % modulus;
            (new_point, new_a, new_b)
        }
        1 => {
            // Point = Point + g
            let new_point = point + g;
            let new_a = (a + BigUint::one()) % modulus;
            let new_b = b;
            (new_point, new_a, new_b)
        }
        2 => {
            // Point = Point + target_point
            let new_point = point + target_point;
            let new_a = a;
            let new_b = (b + BigUint::one()) % modulus;
            (new_point, new_a, new_b)
        }
        _ => unreachable!(),
    }
}
*/