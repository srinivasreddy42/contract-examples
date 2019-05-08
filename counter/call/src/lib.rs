#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::contract_api::call_contract;
use common::contract_api::pointers::ContractPointer;

#[no_mangle]
pub extern "C" fn call() {
    //This hash comes from blake2b256( [0;32] ++ [0;8] ++ [0;4] )
    let hash = ContractPointer::Hash([
        173, 137, 49, 254, 60, 66, 211, 215, 12, 127, 32, 84, 119, 180, 68, 191, 87, 33,
        97, 3, 244, 119, 4, 74, 206, 246, 78, 153, 188, 96, 125, 71
    ]);
    let arg = "inc";
    let _result: () = call_contract(hash.clone(), &arg, &Vec::new());
    let value: i32 = {
        let arg = "get";
        call_contract(hash, &arg, &Vec::new())
    };
    assert_eq!(value, 1);
}
