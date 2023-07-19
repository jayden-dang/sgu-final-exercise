use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
  };
  
  #[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
  }
  