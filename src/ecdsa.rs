use std::str::FromStr;

use secp256k1::{ 
    PublicKey, Secp256k1, Message, Signature
};
use sha2::{Sha256, Digest};

pub const PUBLIC_KEY: &str = "0258e3c1a0e88142e931da2ace5df1cca2f429dbb83ed1b3c9563bd7f6df0126b4";

pub fn verify(data: &[u8], sig: &str) -> bool {
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_str(PUBLIC_KEY).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let msg = Message::from_slice(&hasher.finalize()).unwrap();
    
    let mut sig = Signature::from_str(&sig).unwrap();
    let s = sig.normalize_s();
    let res = secp.verify(&msg, &sig, &public_key);
    if res.is_ok() {
        true
    }else {
        false
    }
}