use std::result;
use error::EnclaveError;
use sgx_types::sgx_status_t;
use sgx_tservice::sgxtime::SgxTime;
use pse_session::{create_pse_session, close_pse_session};

type Result<T> = result::Result<T, EnclaveError>;

#[no_mangle]
pub extern "C" fn sgx_time_sample() -> sgx_status_t { // FIXME: get rid of this eventually

    match create_pse_session() {
        Ok(_) => println!("Create PSE session done"),
        _ => {
            println!("Cannot create PSE session");
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        }
    }
    let ttime = SgxTime::now();
    match ttime {
        Ok(st) => println!("Ok with {:?}", st),
        Err(x) => {
            println!("Err with {}", x);
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        }
    }
    match close_pse_session(0) { // NOTE: 0 here literally because of my own version of close session. Ignore!
        Ok(_) => println!("close PSE session done"),
        _ => {
            println!("Cannot close PSE session");
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        }
    }
    sgx_status_t::SGX_SUCCESS
}

fn get_sgx_time() -> Result<SgxTime> {
    create_pse_session()
        .and_then(get_sgx_time_struct)
        .and_then(close_pse_session)
}

fn get_sgx_time_struct<T>(_t: T) -> Result<SgxTime> {
     Ok(SgxTime::now()?)
}
