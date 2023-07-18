pub mod metadata;
pub mod ecommerce;

use metadata::NFTContractMetadata;
use ecommerce::{Shop, Product};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,  PanicOnDefault)]
pub struct Contract {
  pub owner_id: AccountId,
  pub shops: UnorderedMap<String, Shop>,
  pub products: UnorderedMap<String, Product>,
  pub metadata: Option<NFTContractMetadata>,
}


#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    let metdata = NFTContractMetadata {
      spec: "nft-1.0.0".to_string(),
      name: "E-Commerce HuyDo Contract".to_string(),
    };

    Self {
      owner_id: env::signer_account_id(),
      shops: UnorderedMap::new(b"shops".to_vec()),
      products: UnorderedMap::new(b"products".to_vec()),
      metadata: Some(metdata),
    }
  }
}

