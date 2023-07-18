use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

pub type ShopId = AccountId;
pub type ProductId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
  pub product_id: ProductId,
  pub shop_id: ShopId,
  pub name: String,
  pub total_supply: u64,
  pub price: Balance,
  pub description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Shop {
  pub shop_id: ShopId,
  pub name: String,
  pub description: String,
  pub total_product: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
  ProductPerOwnerKey,
  ProductById,
  Products,
  Shops,
}

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub ecommerce_name: String,
  pub shops: LookupMap<ShopId, Shop>,
  pub products: LookupMap<ProductId, Product>,
  pub products_per_shop: UnorderedMap<ShopId, Vec<Product>>,
  pub total_shops: u128,
  pub total_products: u128,
}

// Outsourcing or E-Commerce
pub trait ImplementOutsourcing {
  fn create_user();
  fn take_job();
  fn create_client();
  fn create_job();
  fn view_job_by_id();
  fn view_all_jobs_per_client();
  fn view_all_jobs();
  fn payment(); // Payment -> Jobs will remove from list
}

// -> Thứ 3 tuần sau
pub trait ImplementECommerce {
  fn create_shop(&mut self, name: String, description: String) -> Shop;
  fn create_product(&mut self, name: String, total_supply: u64, price: Balance, description: String) -> Product;
  fn view_all_products(&self) -> Vec<Product>;
  fn view_all_products_per_shop(&self, shop_id: ShopId) -> Vec<Product>;
  fn view_product_by_id(&self, product_id: ProductId) -> Product;
  fn payment(&mut self, product_id: ProductId) -> Promise;
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      ecommerce_name: "POPU_SHOP".to_string(),
      shops: LookupMap::new(StorageKey::Shops.try_to_vec().unwrap()),
      products: LookupMap::new(StorageKey::Products.try_to_vec().unwrap()),
      products_per_shop: UnorderedMap::new(StorageKey::ProductPerOwnerKey.try_to_vec().unwrap()),
      total_shops: 0,
      total_products: 0,
    }
  }

  pub fn get_total_shops(&self) -> u128 {
    self.total_shops
  }

  pub fn get_total_products(&self) -> u128 {
    self.total_products
  }
}

// Implement the contract structure
#[near_bindgen]
impl ImplementECommerce for Contract {
  fn create_shop(&mut self, name: String, description: String) -> Shop {
    let owner = env::signer_account_id();
    assert!(!self.shops.contains_key(&owner), "Shop already exists");

    self.total_shops = self.total_shops + 1;

    let shop = Shop { shop_id: env::signer_account_id(), name, description, total_product: 0 };

    self.shops.insert(&owner, &shop);

    shop
  }

  fn create_product(&mut self, name: String, total_supply: u64, price: Balance, description: String) -> Product {
    let owner = env::signer_account_id();
    assert!(self.shops.contains_key(&owner), "Your shop not exists");

    let total_product = self.total_products + 1;
    let product_id = total_product.to_string();

    let product = Product {
      product_id: product_id.clone(),
      name,
      total_supply,
      price,
      description,
      shop_id: env::signer_account_id(),
    };

    let mut products_set: Vec<Product> = self.products_per_shop.get(&owner).unwrap_or_else(|| Vec::new());
    products_set.push(product.clone());

    self.products_per_shop.insert(&owner, &products_set);
    self.products.insert(&product_id, &product);

    self.total_products = self.total_products + 1;

    product
  }

  fn view_all_products(&self) -> Vec<Product> {
    let mut all_products: Vec<Product> = Vec::new();

    for i in 1..self.total_products + 1 {
      if let Some(product) = self.products.get(&(i.to_string())) {
        all_products.push(product);
      }
    }

    all_products
  }

  fn view_all_products_per_shop(&self, shop_id: ShopId) -> Vec<Product> {
    self.products_per_shop.get(&shop_id).unwrap()
  }

  fn view_product_by_id(&self, product_id: ProductId) -> Product {
    self.products.get(&product_id).unwrap()
  }

  #[payable]
  fn payment(&mut self, product_id: ProductId) -> Promise {
    let mut product = self.products.get(&product_id).unwrap();
    let price = product.price;
    assert!(price < env::attached_deposit(), "Not equal the price");

    product.total_supply = product.total_supply - 1;

    self.products.insert(&product_id, &product);

    Promise::new(product.shop_id).transfer(price)
  }
}
