// pub mod metadata;
use crate::Contract;
use crate::*;
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::env;
use near_sdk::AccountId;
use near_sdk::Balance;
use near_sdk::Promise;

pub trait ImplementECommerce {
  fn create_shop(&mut self, shop_id: AccountId, name: String, decription: String);
  fn create_product(&mut self, product_id: String, shop_id: AccountId, name: String, price: Balance, total_supply: u64);
  fn view_all_products(&self) -> Vec<Product>;
  fn view_all_products_per_shop(&self, shop_id: AccountId) -> Vec<Product>;
  fn view_product_by_id(&self, product_id: String) -> Option<Product>;
  fn update_shop(&mut self,shop_id: AccountId, name: String, decription: String) -> ShopMetadata;
  fn update_product(&mut self, product_id: String, shop_id: AccountId, name: String, price: Balance, total_supply: u64) ->Product;
  fn payment(&mut self, product_id: String, amount: u64) -> Promise;
}

#[near_bindgen]
impl ImplementECommerce for Contract {
  fn create_shop(&mut self, shop_id: AccountId, name: String, decription: String) {
    assert!(self.shops.get(&shop_id).is_none(), "Shop already exists");
    let owner = env::signer_account_id();
    let shop = ShopMetadata { shop_id: shop_id.clone(), name, decription, owner };
    self.shops.insert(&shop_id, &shop);
  }

  fn create_product(
    &mut self,
    product_id: String,
    shop_id: AccountId,
    name: String,
    price: Balance,
    total_supply: u64,
  ) {
    assert!(self.products.get(&product_id).is_none(), "Product already exists");
    assert!(self.shops.get(&shop_id).is_some(), "Shop does not exist");

    let product = Product { product_id: product_id.clone(), shop_id, name, price, total_supply };
    self.products.insert(&product_id, &product);
  }

  fn view_all_products(&self) -> Vec<Product> {
    self.products.values().collect()
  }

  fn view_all_products_per_shop(&self, shop_id: AccountId) -> Vec<Product> {
    self.products.values().filter(|product| product.shop_id == shop_id).collect()
  }

  fn view_product_by_id(&self, product_id: String) -> Option<Product> {
    self.products.get(&product_id)
  }

  fn update_shop(&mut self,shop_id: AccountId, name: String, decription: String) -> ShopMetadata {
    let shop = self.shops.get(&shop_id).expect("Shop does not exist");
    assert!(shop.owner.eq(&env::signer_account_id()),"you are not the shop owner");

    let new_shop = ShopMetadata {shop_id, name, decription, owner: shop.owner };
    self.shops.insert(&shop.shop_id, &new_shop);
    new_shop
  }

  fn update_product(&mut self, product_id: String, shop_id: AccountId, name: String, price: Balance, total_supply: u64) ->Product {
    let product = self.products.get(&product_id).expect("Product does not exist");
    let owner = self.shops.get(&product.shop_id).expect("Shop does not exist").owner;
    assert!(owner.eq(&env::signer_account_id()),"you are not the shop owner");

    let new_product = Product { product_id: product_id.clone(), shop_id, name, price, total_supply };
    self.products.insert(&product_id, &new_product);
    new_product
  }

  #[payable]
  fn payment(&mut self, product_id: String, amount: u64) -> Promise {
    let mut product = self.products.get(&product_id).expect("Product does not exist");
    assert!(product.total_supply > amount, "Product out of stock");
    let price = (product.price * amount as u128) * 10u128.pow(24);
    assert!(price == env::attached_deposit(), "Not equal the price");

    product.total_supply -= amount;
    self.products.insert(&product_id, &product);
    let owner = self.shops.get(&product.shop_id).expect("Product does not exist");
    Promise::new(owner.owner.clone()).transfer(price)
  }
}
