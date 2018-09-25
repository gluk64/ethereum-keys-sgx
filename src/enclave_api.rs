#![allow(improper_ctypes)]
use secp256k1::key::PublicKey;
use sgx_types::{sgx_enclave_id_t, sgx_status_t};

extern {
    pub fn generate_keypair(
        eid: sgx_enclave_id_t, 
        retval: *mut sgx_status_t, 
        pub_key: *mut PublicKey, 
        sealed_log: *mut u8,
        sealed_log_size: *const u32
    ) -> sgx_status_t;

    pub fn sign_message(
        eid: sgx_enclave_id_t, 
        retval: *mut sgx_status_t, 
        sealed_log: *mut u8,
        sealed_log_size: *const u32,
        hashed_message: *mut u8
    ) -> sgx_status_t;
}