#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

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
  fn create_shop();
  fn create_product();
  fn view_all_products();
  fn view_all_products_per_shop();
  fn view_product_by_id();
  fn payment(); // Payment -> Product decrement total_supply;
}

// Implement the contract structure
#[near_bindgen]
impl Contract {}
