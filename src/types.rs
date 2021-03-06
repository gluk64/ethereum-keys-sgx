use std::mem::size_of;
use secp256k1::key::{PublicKey, SecretKey};
use sgx_types::{sgx_sealed_data_t, sgx_mc_uuid_t, sgx_time_t, sgx_time_source_nonce_t};

#[allow(dead_code)]
pub struct MonotonicCounter {
    value: u32,
    id: sgx_mc_uuid_t,
}

#[allow(dead_code)]
pub struct SgxTime {
    timestamp: sgx_time_t,
    source_nonce: sgx_time_source_nonce_t,
}

#[allow(dead_code)]
pub struct KeyStruct {
    sgx_time: SgxTime,   
    public: PublicKey,
    secret: SecretKey,
    accesses_mc: MonotonicCounter,
    signatures_mc: MonotonicCounter
}

pub type Hash = [u8;32];
pub type Signature = [u8;65];
pub type EncryptedKeyStruct = Vec<u8>;
pub static ENCRYPTED_KEYPAIR_SIZE: usize = size_of::<sgx_sealed_data_t>() + size_of::<KeyStruct>();
