#![allow(dead_code)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};
use near_sdk::ext_contract;

// Define the contract structure
pub type JobId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  pub user_id: AccountId,
  pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Client {
  pub client_id: AccountId,
  pub name: String,
  pub number_of_job : u128  , 

}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  pub job_owner : AccountId  , 
  pub job_id :  JobId,
  pub desc : String,
  pub pay  : u128  ,  
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize ,PanicOnDefault)]
pub struct Contract {
  pub platform_name: AccountId ,
  pub user : LookupMap<AccountId, User>,
  pub client  :  LookupMap<AccountId, Client> , 
  pub user_taken_job :  UnorderedMap<AccountId , JobId> , 
  pub job_by_id : LookupMap<JobId, Job> , 
  pub job_by_client : UnorderedMap<AccountId, Vec<Job> >  ,  
  pub job_arr  : UnorderedMap<u128,Job > , 
  pub total_job: u128 , 
  pub total_user : u128  ,
  pub total_client : u128 , 

}

// Outsourcing or E-Commerce
pub trait ImplementOutsourcing {
  fn create_user();// lookupmap 
  fn take_job();// unordermap 
  fn create_client();//lookup map 
  fn create_job();//unordermap 
  fn view_job_by_id();//lookup map 
  fn view_all_jobs_per_client(); // unordermap 
  fn view_all_jobs();//unordermap 
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

impl Contract {
  #[init] // this is to show that this function is needed to init the contract 
  pub fn init() -> Self {
    Self {
      platform_name: env::signer_account_id() ,
      user : LookupMap::new(b"user_contract".try_to_vec().unwrap()),
      client  :  LookupMap::new(b"client_contract".try_to_vec().unwrap()) , 
      job_arr  :UnorderedMap::new(b"client".try_to_vec().unwrap())  , 
      user_taken_job :  UnorderedMap::new(b"takent_job".try_to_vec().unwrap()) , 
      job_by_id : LookupMap::new(b"job_id".try_to_vec().unwrap()), 
      job_by_client : UnorderedMap::new(b"job_by_client".try_to_vec().unwrap()) ,  
      total_job: 0 , 
      total_user : 0   ,
      total_client : 0 , 

    
    }// a new contract is initialize 
  }
  //create the owner account 
  pub fn create_user(& mut self  , name :String ) -> User
  {
    let sign_user =  env::signer_account_id()  ; 
    assert!(!self.user.contains_key(&sign_user), "user already exists");
    let total_user = self.total_user + 1; 
    
    let user = User { user_id: env::signer_account_id(), name};
    self.user.insert(&sign_user , &user  )  ; 

    user

  }
  pub fn create_client(& mut self  , name :String ) -> Client
  {
    let sign_client =  env::signer_account_id()  ; 
    assert!(!self.client.contains_key(&sign_client), "client already exists");

    let client = Client{ client_id: env::signer_account_id(), name  , number_of_job : 0};
    self.client.insert(&sign_client , &client  )  ; 

    client
  }
  pub fn create_job(&mut self , id :JobId  , desc: String , pay : u128 ) -> Job
  {
    let job_owner =  env::signer_account_id()  ; 
    // check if the client exits 
    assert!(self.client.contains_key(&job_owner), "you are not sign in as  a client");
    let job: Job =  Job { job_owner: env::signer_account_id()  ,    job_id: id.clone(),desc  , pay } ; 
    let mut job_set: Vec<Job> = self.job_by_client.get(&job_owner).unwrap_or_else(|| Vec::new());
    let  total_job = self.total_job +1  ;  
    self.job_arr.insert(&total_job ,&job ) ; 
    self.total_client = self.total_client +1  ;
    job_set.push(job.clone()); 
    self.job_by_client.insert(&job_owner  ,  &job_set)  ; 
    self.job_by_id.insert(&id,&job )  ; 
    job
  }
  pub fn get_job_by_id(&self, job_id: JobId) ->  Job {
    self.job_by_id.get(&job_id).unwrap()
  }
  pub fn take_job(&mut self    , job_id : JobId ) -> Job 
  {
    let user_id = env::signer_account_id()  ; 
    assert!(self.client.contains_key(&user_id), "you have not sign in as a user" )  ; 
    self.user_taken_job.insert(&user_id, &job_id ); 
    self.get_job_by_id(job_id) 
  }
  pub fn  view_all_jobs(&self) ->Vec<Job>
  {
    let mut all_products: Vec<Job> = Vec::new();
    
    for i in 1..self.job_arr.len() + 1 {
      if let Some(product) = self.job_arr.get(&(i as u128)) {
        all_products.push(product); //loop thourght the product and print them out 
      }
    }

    all_products
  }
  #[payable]
  pub fn payment(&mut self, job_id : JobId)-> Promise {
    let mut job = self.get_job_by_id(job_id);
    let user = env::signer_account_id()   ; 
    assert!(self.user.contains_key(&user), "you have not sign up to be a user");
    assert_eq!(self.user_taken_job.get(&user).unwrap(), job.job_id,"the user have not taken the job")  ;
    let pay = job.pay  ;

    assert_eq!(pay, env::attached_deposit(), "Not Correct price");


    Promise::new(self.platform_name.clone()).transfer(pay)
  }
}

// command 
// cargo make call create_user '{"name" : "eamon shop"}' --account-id konodioda2411.testnet
// cargo make call create_user '{"name" : "eamon shop"}' --account-id dio.konodioda2411.testnet

 //
 
