use hex;

use crate::blake2b::Blake2bHasher;
use crate::h256::H256;
use crate::merkle_proof::CompiledMerkleProof;

pub struct MerkleVerify;

impl MerkleVerify {
    pub fn verify_proof(proof: &str, root: &str, leaves: &Vec<(&str, &str)>) -> bool {
        let proof = hex::decode(proof).unwrap();
        let root: [u8; 32] = hex::decode(root).unwrap().try_into().expect("failed");
        let leaves = leaves.into_iter().map(|l| {
            let left: [u8; 32] = hex::decode(l.0).unwrap().try_into().expect("failed");
            let right: [u8; 32] = hex::decode(l.1).unwrap().try_into().expect("failed");
            (H256::from(left), H256::from(right))
        }).collect::<Vec<_>>();

        let proof = CompiledMerkleProof(proof);
        let root = H256::from(root);
        let result = proof.verify::<Blake2bHasher>(&root, leaves).unwrap();

        return result;
    }
}

