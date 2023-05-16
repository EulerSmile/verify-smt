use std::hash;

use blake2::{digest::FixedOutput, Blake2bMac};
use hex_literal::hex;
use blake2b_rs::Blake2bBuilder;
use digest::consts::U32;
use smt::{blake2b::{Blake2bMac256, Blake2bHasher as SMTBlake2b}, traits::Hasher, h256::H256};

#[test]
#[rustfmt::skip]
fn blake2b_with_key() {
    let key = hex!("
        000102030405060708090a0b0c0d0e0f
        101112131415161718191a1b1c1d1e1f
    ");
    
    let persona = b"personal";
    let ctx = Blake2bMac256::new_with_salt_and_personal(&key, &[], persona).unwrap();
    let out: [u8; 32] = ctx.finalize_fixed()[..].try_into().expect("slice with incorrect length");
    println!("blake2: {:?}", out);
    // [216, 141, 253, 59, 178, 225, 251, 57, 240, 43, 71, 16, 202, 233, 139, 24, 125, 156, 220, 66, 65, 239, 91, 142, 82, 244, 220, 155, 221, 254, 57, 126]

    let mut out = [0u8; 32];
    let mut hasher = Blake2bBuilder::new(32).salt(&[]).personal(persona).key(&key).build();
    hasher.finalize(&mut out);
    println!("blake2b-rs: {:?}", out);
    // [216, 141, 253, 59, 178, 225, 251, 57, 240, 43, 71, 16, 202, 233, 139, 24, 125, 156, 220, 66, 65, 239, 91, 142, 82, 244, 220, 155, 221, 254, 57, 126]
}

#[test]
fn blake2b_without_key() {
    const PERSONALIZATION: &[u8] = b"sparsemerkletree";
    let salt: &[u8] = &[];
    let key: &[u8] = &[];
    let ctx = Blake2bMac256::new_with_salt_and_personal(key, salt, PERSONALIZATION).unwrap();
    let out: [u8; 32] = ctx.finalize_fixed()[..].try_into().expect("slice with incorrect length");
    println!("blake2: {:?}", out);
    // [146, 200, 238, 190, 99, 195, 209, 158, 61, 186, 253, 78, 231, 206, 122, 240, 212, 148, 183, 192, 123, 169, 250, 191, 184, 108, 209, 54, 138, 246, 132, 144]

    let mut out = [0u8; 32];
    let mut hasher = Blake2bBuilder::new(32).key(key).salt(salt).personal(PERSONALIZATION).build();
    hasher.finalize(&mut out);
    println!("blake2b-rs: {:?}", out);
    // [86, 76, 113, 0, 182, 253, 107, 161, 250, 165, 230, 88, 221, 238, 116, 125, 65, 247, 12, 41, 121, 183, 107, 240, 56, 1, 197, 104, 91, 49, 237, 252]
}

#[test]
fn blake2b_hasher() {
    let b = 1_u8;
    let h = H256::from([0; 32]);
    let mut hasher = SMTBlake2b::default();
    // hasher.write_byte(b);
    // hasher.write_h256(&h);
    let r = hasher.finish();
    println!("r: {:?}", r);
}