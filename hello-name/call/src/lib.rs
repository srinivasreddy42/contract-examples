#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::contract_api::{call_contract, new_uref};
use common::contract_api::pointers::ContractPointer;
use common::value::Value;

#[no_mangle]
pub extern "C" fn call() {
    //This hash comes from blake2b256( [0;32] ++ [0;8] ++ [0;4] )
    let hash = ContractPointer::Hash([
        212, 41, 31, 85, 105, 151, 130, 113, 199, 144, 151, 41, 197, 34, 171, 217, 245,
        220, 171, 42, 50, 159, 118, 156, 38, 188, 236, 206, 150, 76, 203, 172
    ]);
    let arg = "World";
    let result: String = call_contract(hash, &arg, &Vec::new());
    assert_eq!("Hello, World", result);

    //store the result at a uref so it can be seen as an effect on the global state
    let _uref = new_uref(Value::String(result));
}
