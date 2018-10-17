use std::result;
use sgx_types::*;
use keygen::KeyPair;
use error::EnclaveError;
use sgx_tseal::SgxSealedData;
use sgx_types::marker::ContiguousMemory;

type Result<T> = result::Result<T, EnclaveError>;

#[allow(dead_code)]
pub fn seal_keypair_with_additional_data(sealed_log: * mut u8, sealed_log_size: u32, kp: KeyPair, add_data_slice: &[u8]) -> Result<sgx_status_t> {
    Ok(seal_keypair(sealed_log, sealed_log_size, kp, add_data_slice)?)
}

pub fn seal_keypair_no_additional_data(sealed_log: * mut u8, sealed_log_size: u32, kp: KeyPair) -> Result<sgx_status_t> {
    Ok(seal_keypair(sealed_log, sealed_log_size, kp, &[0_u8; 0])?)
}

fn seal_keypair(sealed_log: * mut u8, sealed_log_size: u32, kp: KeyPair, add_data_slice: &[u8]) -> Result<sgx_status_t> {
    match to_sealed_log(&SgxSealedData::<KeyPair>::seal_data(&add_data_slice, &kp)?, sealed_log, sealed_log_size) { // FIXME: Make more functional & less gross...
        Some(_) => Ok(sgx_status_t::SGX_SUCCESS),
        None    => Err(EnclaveError::SGXError(sgx_status_t::SGX_ERROR_INVALID_PARAMETER))
    }
}

pub fn to_sealed_log<T: Copy + ContiguousMemory>(
    sealed_data: &SgxSealedData<T>, 
    sealed_log: * mut u8,
    sealed_log_size: u32
) -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data.to_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

pub fn from_sealed_log<'a, T: Copy + ContiguousMemory>(
    sealed_log: * mut u8, 
    sealed_log_size: u32
) -> Option<SgxSealedData<'a, T>> {
    unsafe {
        SgxSealedData::<T>::from_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}


