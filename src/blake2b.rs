use crate::traits::Hasher;
use blake2::{digest::FixedOutput, Blake2bMac};
use digest::{consts::U32, Mac};

pub type Blake2bMac256 = Blake2bMac<U32>;

pub struct Blake2bHasher(Blake2bMac256);

const PERSONALIZATION: &[u8] = b"sparsemerkletree";

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bMac256::new_with_salt_and_personal(&[], &[], PERSONALIZATION).unwrap();
        Self(blake2b)
    }
}

impl Hasher for Blake2bHasher {
    fn write_byte(&mut self, b: u8) {
        self.0.update(&[b][..]);
    }

    fn write_h256(&mut self, h: &crate::h256::H256) {
        self.0.update(h.as_slice());
    }

    fn finish(self) -> crate::h256::H256 {
        let out: [u8; 32] = self.0.finalize_fixed()[..].try_into().expect("slice with incorrect length");
        out.into()
    }
}