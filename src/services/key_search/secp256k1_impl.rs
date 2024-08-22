// use num_bigint::{BigUint, ToBigUint};
// use num_traits::{Num, One, Zero};
// use std::collections::HashMap;
// use std::time::Instant;
// use hex;
//
// // SECP256k1 CONSTANTS
// const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
// const A: &str = "0000000000000000000000000000000000000000000000000000000000000000";
// const B: &str = "0000000000000000000000000000000000000000000000000000000000000007";
// const ORDER: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
// const G_X: &str = "79BE667EF9DCBBAC55A06295CE870B70B8EFC9D6B8F2608F48E3B7C8D759F57";
// const G_Y: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08F76A4D3A4";
//
// struct Point {
//     x: BigUint,
//     y: BigUint,
// }
//
// impl Point {
//     fn new(x: BigUint, y: BigUint) -> Self {
//         Point {
//             x: x.clone(),
//             y: y.clone(),
//         }
//     }
// }
//
// fn mod_sqrt(a: &BigUint, p: &BigUint) -> Option<BigUint> {
//     if a.is_zero() {
//         return Some(BigUint::zero());
//     }
//     if p == &BigUint::from(2u32) {
//         return Some(a.clone() % p);
//     }
//
//     // Tonelli-Shanks algorithm initialization
//     let mut s = p - 1u32.to_biguint().unwrap();
//     let mut q = BigUint::one();
//     while &s % 2u32.to_biguint().unwrap() == BigUint::zero() {
//         s /= 2u32.to_biguint().unwrap();
//         q *= 2u32.to_biguint().unwrap();
//     }
//
//     let mut z = BigUint::from(2u32);
//     while mod_exp(&z, &(p - 1u32.to_biguint().unwrap()) / 2u32.to_biguint().unwrap(), p) != p - 1u32.to_biguint().unwrap() {
//         z += 1u32.to_biguint().unwrap();
//     }
//
//     let mut m = q.clone();
//     let mut t = mod_exp(a, &(p + 1u32.to_biguint().unwrap()) / 4u32.to_biguint().unwrap(), p);
//     let mut r = t.clone();
//
//     while t != BigUint::one() {
//         let mut i = 1u32;
//         let mut t2i = t.clone();
//         while t2i != BigUint::one() {
//             t2i = mod_exp(&t2i, BigUint::from(2u32), p);
//             i += 1;
//         }
//
//         let mut b = mod_exp(&z, BigUint::from(2u32).pow(u32::try_from(&(m - i.to_biguint().unwrap() - 1u32.to_biguint().unwrap())).unwrap()), p);
//         m = i.to_biguint().unwrap();
//         t = mod_exp(&t, &(p + 1u32.to_biguint().unwrap()) / 2u32.to_biguint().unwrap(), p);
//         r = (r * b.clone()) % p;
//         z = mod_exp(&b, BigUint::from(2u32), p);
//     }
//
//     Some(r)
// }
//
//
// fn mod_exp(base: &BigUint, exp: BigUint, modulus: &BigUint) -> BigUint {
//     base.modpow(&exp, modulus)
// }
//
// fn bsgs(target_point: &Point, g: &Point, max_steps: usize) -> Option<BigUint> {
//     let mut baby_steps = HashMap::new();
//     let mut current = Point::new(BigUint::zero(), BigUint::zero());
//     let p = BigUint::from_str_radix(P, 16).unwrap();
//     let g_x = g.x.clone();
//     let g_y = g.y.clone();
//
//     for i in 0..max_steps {
//         baby_steps.insert((current.x.clone(), current.y.clone()), i.to_biguint().unwrap());
//         current = Point::new(
//             ((current.x.clone() + &g_x) % &p),
//             ((current.y.clone() + &g_y) % &p)
//         );
//     }
//
//     let max_steps_biguint = max_steps.to_biguint().unwrap();
//     let giant_stride = Point::new(
//         (g_x * &max_steps_biguint),
//         (g_y * &max_steps_biguint)
//     );
//
//     let mut current = target_point.clone();
//     for j in 0..max_steps {
//         if let Some(&ref i) = baby_steps.get(&(current.x.clone(), current.y.clone())) {
//             return Some(j.to_biguint().unwrap() * max_steps_biguint + i);
//         }
//         let current = &Point::new(
//             ((current.x.clone() - &giant_stride.x) % &p),
//             ((current.y.clone() - &giant_stride.y) % &p)
//         );
//     }
//     None
// }
//
// pub fn main() {
//     let start_time = Instant::now();
//
//     // Chave pública fornecida
//     let target_public_key_hex = "033c4a45cbd643ff97d77f41ea37e843648d50fd894b864b0d52febc62f6454f7c";
//
//     println!("target_public_key_hex");
//
//     let target_public_key_bytes = hex::decode(target_public_key_hex).unwrap();
//     let public_key_x = BigUint::from_bytes_be(&target_public_key_bytes[1..33]);
//     let public_key_y_square = (&public_key_x.modpow(&BigUint::from(3u32), &BigUint::from_str_radix(P, 16).unwrap())
//         + BigUint::from_str_radix(A, 16).unwrap() * &public_key_x
//         + BigUint::from_str_radix(B, 16).unwrap()) % BigUint::from_str_radix(P, 16).unwrap();
//     let public_key_y = mod_sqrt(&public_key_y_square, &BigUint::from_str_radix(P, 16).unwrap()).unwrap();
//
//     println!("2222222222222222");
//
//     let public_key_y = if (target_public_key_hex.starts_with("02") && &public_key_y % 2u32 != BigUint::try_from(0).unwrap())
//         || (target_public_key_hex.starts_with("03") && &public_key_y % 2u32 == BigUint::try_from(0).unwrap()) {
//         BigUint::from_str_radix(P, 16).unwrap() - public_key_y
//     } else {
//         public_key_y
//     };
//
//     println!("33333333333");
//
//     let target_public_key_point = Point::new(public_key_x, public_key_y);
//     let g = Point::new(BigUint::from_str_radix(G_X, 16).unwrap(), BigUint::from_str_radix(G_Y, 16).unwrap());
//
//     // Configurações do puzzle
//     let max_steps = 2usize.pow(20u32);
//
//     println!("44444444444444444");
//
//     // Executando BSGS para encontrar a chave privada
//     match bsgs(&target_public_key_point, &g, max_steps) {
//         Some(private_key_integer) => {
//             let private_key_hex = private_key_integer.to_str_radix(16);
//             println!("Chave privada encontrada: {:064x}", private_key_integer);
//             println!("WIF: {}", private_key_hex); // Substitua por sua própria função para gerar WIF
//             // println!("Endereço público: {}", generate_public(&private_key_hex)); // Substitua por sua própria função para gerar endereço público
//         }
//         None => {
//             println!("Chave privada não encontrada.");
//         }
//     }
//
//     println!("5555555555555");
//
//     let elapsed_time = start_time.elapsed();
//     println!("Tempo total: {:.2?} segundos", elapsed_time);
// }
