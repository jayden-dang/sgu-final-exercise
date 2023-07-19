  use crate::{Contract, ContractExt};

  use near_sdk::{env, near_bindgen, AccountId, Promise};
  use near_sdk::serde::{Deserialize, Serialize};
  use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
  use std::collections::HashSet;

  #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct Product {
    pub product_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: u128,
    pub quantity: u128,
    pub image: Option<String>,
  }

  #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
  #[serde(crate = "near_sdk::serde")]
  pub struct Shop {
    pub shop_id: String,
    pub owner_id: AccountId,
    pub products: HashSet<String>,
  }

  pub trait ImplementECommerce {
    fn create_shop(&mut self, shop_id: String, owener_id: AccountId, products: HashSet<String>)-> Option<Shop>;
    fn create_product(&mut self, shop_id: String, product_id: String, title: Option<String>, price: u128,
       description: Option<String>, quantity: u128, image: Option<String>) -> Option<Product>;
    fn view_shop_by_id(&self, shop_id: String) -> Option<Shop>;
    fn view_all_products(&self) -> Vec<Product>;
    fn view_all_products_per_shop(&self, shop_id: String) -> Vec<Product>;
    fn view_product_by_id(&self, product_id: String) -> Option<Product>;
    fn view_all_shops(&self) -> Vec<Shop>;
    fn update_product(&mut self, product_update: Product)-> Option<Product>;
    fn payment(&mut self, product_id: String, quantity: u128)-> Promise;
  }

  #[near_bindgen]
  impl ImplementECommerce for Contract {
    //Them moi shop
    fn create_shop(
      &mut self,
      shop_id: String,
      owner_id: AccountId,
      products: HashSet<String>) -> Option<Shop>{
        let exist_shop = self.shops.get(&shop_id);
        if exist_shop.is_none() {
          let shop = Shop{shop_id, owner_id, products};
          self.shops.insert(&shop.shop_id, &shop);
          return Some(shop);
        }       
        return None;   
    }

    //Xem shop bang ID
    fn view_shop_by_id(&self, shop_id: String) -> Option<Shop>{
      self.shops.get(&shop_id)
    }

    //Them moi san pham vao shop
    fn create_product(
      &mut self,
      shop_id: String,
      product_id: String,
      title: Option<String>, price: u128,
      description: Option<String>,
      quantity: u128,
      image: Option<String>) -> Option<Product>{
        let exist_product = self.products.get(&product_id);
        if exist_product.is_none() {
          //Them san pham vao danh sach
          let product = Product {product_id, title, price, description, quantity, image};
          self.products.insert(&product.product_id, &product);
          //Them ID san pham vao shop
          let mut shop = self.shops.get(&shop_id).unwrap();
          shop.products.insert(product.product_id.clone());
          self.shops.insert(&shop_id, &shop);         
          //return
          return Some(product);
        }       
        return None;
    }

    //Xem tat ca san pham
    fn view_all_products(&self) -> Vec<Product>{
      let mut all_products = Vec::new();
      for i in self.products.values() {
        all_products.push(i);
      }
      all_products
    }

    //Xem san pham theo shop
    fn view_all_products_per_shop(&self, shop_id: String) -> Vec<Product>{
      let mut all_products_per_shop = Vec::new();
      let exist_shop = self.shops.get(&shop_id);
      for i in exist_shop.unwrap().products {
        all_products_per_shop.push(self.products.get(&i).unwrap());
      }
      all_products_per_shop
    }

    //Xem san pham bang ID
    fn view_product_by_id(&self, product_id: String) -> Option<Product>{
      self.products.get(&product_id)
    }

    //Xem tat ca shop 
    fn view_all_shops(&self) -> Vec<Shop>{
      let mut all_shops = Vec::new();
      for i in self.shops.values(){
        all_shops.push(i);
      }
      all_shops
    }

    //Cap nhat san pham
    fn update_product(&mut self, product_update: Product)-> Option<Product>{
      let exist_product = self.products.get(&product_update.product_id);
      if !exist_product.is_none() {
        self.products.insert(&product_update.product_id,&product_update);
        return Some(product_update);
      }
      return None;
    }

    //Payment
    #[payable]
    fn payment(&mut self, product_id: String, quantity: u128)-> Promise {
      let mut product = self.products.get(&product_id).unwrap();
      
      let price = product.price*quantity;
      assert!(price == env::attached_deposit(), "Not enough the price");

      product.quantity -= quantity;
      self.update_product(product);

      let sender_id = env::signer_account_id();
      Promise::new(sender_id).transfer(price)
    }
  }