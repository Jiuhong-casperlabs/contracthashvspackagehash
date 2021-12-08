#![no_std]
#![no_main]

use casper_contract;
use casper_types::{ ContractPackageHash, ContractHash};

#[no_mangle]
pub extern "C" fn call() {
  
    let contracthash =  ContractHash::from_formatted_str("contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8")
        .unwrap();
    //contracthash is: 300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8

      contracthash.to_formatted_string();
    //   contracthash_str is: contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8

      let contractpackagehash = ContractPackageHash::from_formatted_str("contract-package-wasm1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d")
        .unwrap();
    // contractpackagehash is 1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d

      contractpackagehash.to_formatted_string();

}
