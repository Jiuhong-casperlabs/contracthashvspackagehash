#[cfg(test)]
mod tests {
    use casper_types::{
       ContractHash, ContractPackageHash,
    };

    #[test]
    fn should_store_hello_world() {
      let contracthash =  ContractHash::from_formatted_str("contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8")
        .unwrap();
    //contracthash is: 300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8
      println!("contracthash is: {} ",contracthash);
      let contracthash_str = contracthash.to_formatted_string();
    //   contracthash_str is: contract-300094544205F5F99Aa33CD87D8f0F0B391e0E6bc1cfB0ccFbF35067E6faB1F8
      println!("contracthash_str is: {}", contracthash_str);

      let contractpackagehash = ContractPackageHash::from_formatted_str("contract-package-wasm1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d")
        .unwrap();
    // contractpackagehash is 1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621d
      println!("contractpackagehash is {}", contractpackagehash);
      let contractpackagehash_str = contractpackagehash.to_formatted_string();
    //   contractpackagehash_str is contract-package-wasm1ddc8ECf041E3A32C5E92155FE6a8437A55eA0716f2b9d9d2C4Da890a5d9621
      println!("contractpackagehash_str is {}",contractpackagehash_str);
    }

    }

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
