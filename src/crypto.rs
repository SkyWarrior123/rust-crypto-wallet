pub mod hashing {
    use sha256::{digest};
    use sha3::{Digest, Keccak256};
    use ripemd::{Ripemd160, Digest as RIPEDigest};
    use crate::base16;


    // bitcoin
    pub fn hash_sha256(input: &[u8]) -> String {
        let r = digest(input);
        r
    }

    pub fn hash_ripemd(input: &[u8]) -> String {
        let mut h = Ripemd160::new();
        h.update(input);
        let out = h.finalize();
        base16::encode_bytes(&out)

    }

    // ethereum
    pub fn hash_keccak256(input: &[u8]) -> String {
        let mut hasher = Keccak256::default();
        hasher.input(input);
        let out = hasher.result();
        let r = base16::encode_bytes(out).to_uppercase();
        return r;
    }

    pub fn hash_keccak256_str (input: &String) -> String {
        let mut hasher = Keccak256::default();
        hasher.input(&input.clone().into_bytes());
        let out = hasher.result();
        let r = base16::encode_bytes(&out).to_uppercase();
        r
    }

}

pub mod bitcoin {

}

pub mod ethereum {

}

pub mod secp256k1 {
    
}