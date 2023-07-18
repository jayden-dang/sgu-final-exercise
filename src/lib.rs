#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

pub type JobId = String;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  users: UnorderedSet<AccountId>,
  clients: UnorderedSet<AccountId>,
  user_data: UnorderedMap<AccountId, User>,
  client_data: UnorderedMap<AccountId, Client>,
  jobs_per_client: UnorderedMap<AccountId, Vec<Job>>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
  user_id: AccountId,
  user_name: Option<String>,
  user_address: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Client {
  client_id: AccountId,
  client_name: Option<String>,
  client_address: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
  job_id: JobId,
  client_id: AccountId,
  salary: Balance,
  freelancer: Option<AccountId>,
}

// Outsourcing
pub trait ImplementOutsourcing {
  fn create_user(&mut self, user_name: Option<String>, user_address: Option<String>);
  fn take_job(&mut self, job_id: JobId);
  fn create_client(&mut self, client_name: Option<String>, client_address: Option<String>);
  fn create_job(&mut self, job_id: JobId, salary: Balance);
  fn view_job_by_id(&self, job_id: JobId) -> Option<Job>;
  fn view_all_jobs_per_client(&self, client_id: AccountId) -> Option<Vec<Job>>;
  fn view_all_jobs(&self) -> Vec<Job>;
  fn payment(&mut self, job_id: JobId) -> Promise; // Payment -> Jobs will remove from list
  fn get_all_users(&self) -> Vec<User>;
  fn get_all_clients(&self) -> Vec<Client>;
  fn update_user(&mut self, user_name: Option<String>, user_address: Option<String>);
  fn update_client(&mut self, client_name: Option<String>, client_address: Option<String>);
  fn update_job(&mut self, job_id: JobId, salary: Balance);
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    Self {
      users: UnorderedSet::new(b"users".to_vec()),
      clients: UnorderedSet::new(b"clients".to_vec()),
      user_data: UnorderedMap::new(b"user_data".to_vec()),
      client_data: UnorderedMap::new(b"client_data".to_vec()),
      jobs_per_client: UnorderedMap::new(b"jobs_per_client".to_vec()),
    }
  }
}

#[near_bindgen]
impl ImplementOutsourcing for Contract {
  fn create_user(&mut self, user_name: Option<String>, user_address: Option<String>) {
    let user_id = env::signer_account_id();
    assert!(!self.users.contains(&user_id), "User already exists");
    self.users.insert(&user_id);
    let user_data = User { user_id: user_id.clone(), user_name, user_address };
    self.user_data.insert(&user_id, &user_data);
  }

  fn update_user(&mut self, user_name: Option<String>, user_address: Option<String>) {
    let user_id = env::signer_account_id();
    assert!(self.users.contains(&user_id), "You are not a User");
    let mut user = self.user_data.get(&user_id).unwrap();
    if let Some(name) = user_name {
      user.user_name = Some(name);
    }
    if let Some(address) = user_address {
      user.user_address = Some(address);
    }
    self.user_data.insert(&user_id, &user);
  }

  fn get_all_users(&self) -> Vec<User> {
    let mut all_users = Vec::new();
    for u in self.user_data.values() {
      all_users.push(u);
    }
    all_users
  }

  fn take_job(&mut self, job_id: JobId) {
    let user_id = env::signer_account_id();
    assert!(self.users.contains(&user_id), "You are not an User");
    let job_option = self.view_job_by_id(job_id.clone());
    match job_option {
      None => {
        panic!("Job not found!");
      },
      Some(mut job) => match job.freelancer {
        None => {
          job.freelancer = Some(user_id);
          let client_id = job.client_id.clone();
          let mut client_jobs = self.jobs_per_client.get(&client_id).unwrap_or_default();
          let job_index = client_jobs.iter().position(|j| j.job_id == job_id).unwrap();
          client_jobs[job_index] = job;
          self.jobs_per_client.insert(&client_id, &client_jobs);
        },
        Some(freelancer) => {
          panic!("Job has been got by {freelancer}");
        },
      },
    }
  }

  fn create_client(&mut self, client_name: Option<String>, client_address: Option<String>) {
    let client_id = env::signer_account_id();
    assert!(!self.clients.contains(&client_id), "Client already exists");
    self.clients.insert(&client_id);
    let client_data = Client { client_id: client_id.clone(), client_name, client_address };
    self.client_data.insert(&client_id, &client_data);
  }

  fn get_all_clients(&self) -> Vec<Client> {
    let mut all_clients = Vec::new();
    for v in self.client_data.values() {
      all_clients.push(v);
    }
    all_clients
  }

  fn update_client(&mut self, client_name: Option<String>, client_address: Option<String>) {
    let client_id = env::signer_account_id();
    assert!(self.clients.contains(&client_id), "You are not a Client");
    let mut client = self.client_data.get(&client_id).unwrap();
    if let Some(name) = client_name {
        client.client_name = Some(name);
    }
    if let Some(address) = client_address {
        client.client_address = Some(address);
    }
    self.client_data.insert(&client_id, &client);
}

  fn create_job(&mut self, job_id: JobId, salary: Balance) {
    let client_id = env::signer_account_id();
    assert!(self.clients.contains(&client_id), "You are not a Client");
    let job_option = self.view_job_by_id(job_id.clone());
    match job_option {
      Some(_job) => {
        panic!("Job already exists")
      },
      None => {
        let job = Job { job_id, client_id: client_id.clone(), salary, freelancer: None };
        let mut client_jobs = self.jobs_per_client.get(&client_id).unwrap_or_default();
        client_jobs.push(job);
        self.jobs_per_client.insert(&client_id, &client_jobs);
      },
    }
  }

  fn update_job(&mut self, job_id: JobId, salary: Balance) {
    let client_id = env::signer_account_id();
    assert!(self.clients.contains(&client_id), "You are not a Client");
    let job_option = self.view_job_by_id(job_id.clone());
    match job_option {
      Some(mut job) => {
        assert_eq!(job.client_id, client_id, "You are not the owner of this job");
        job.salary = salary;
        let mut client_jobs = self.jobs_per_client.get(&client_id).unwrap_or_default();
        let job_index = client_jobs.iter().position(|j| j.job_id == job_id).unwrap();
        client_jobs[job_index] = job;
        self.jobs_per_client.insert(&client_id, &client_jobs);
      },
      None => {
        panic!("Job not found");
      },
    }
  }

  fn view_job_by_id(&self, job_id: JobId) -> Option<Job> {
    let all_jobs = self.view_all_jobs();
    all_jobs.iter().find(|job| job.job_id == job_id).cloned()
  }

  fn view_all_jobs(&self) -> Vec<Job> {
    let mut all_jobs: Vec<Job> = Vec::new();
    for mut vec_job in self.jobs_per_client.values() {
      all_jobs.append(&mut vec_job);
    }
    all_jobs
  }

  fn view_all_jobs_per_client(&self, client_id: AccountId) -> Option<Vec<Job>> {
    assert!(self.clients.contains(&client_id), "Client does not exists");
    self.jobs_per_client.get(&client_id)
  }

  #[payable]
  fn payment(&mut self, job_id: JobId) -> Promise {
    let owner_id = env::signer_account_id();
    let job_option = self.view_job_by_id(job_id.clone());
    match job_option {
      None => {
        panic!("Job not found!")
      },
      Some(job) => {
        assert!(owner_id == job.client_id, "You are not the owner of this job");
        // Kiểm tra số tiền gửi kèm với giao dịch
        let attached_deposit: Balance = env::attached_deposit();
        let salary: Balance = job.salary.into(); // Chuyển salary thành Balance
        let salary_yocto_near: Balance = (salary * 1_000_000_000_000_000_000_000_000) as u128;
        // Kiểm tra số tiền gửi kèm có đúng bằng với salary hay không
        assert_eq!(attached_deposit, salary_yocto_near, "Amount must be equal to salary");
        match job.freelancer {
          None => {
            panic!("Job has no recipients")
          },
          Some(freelancer) => {
            let mut client_jobs = self.jobs_per_client.get(&owner_id).unwrap_or_default();
            let job_index = client_jobs.iter().position(|j| j.job_id == job_id).unwrap();
            client_jobs.remove(job_index);
            self.jobs_per_client.insert(&owner_id, &client_jobs);
            Promise::new(freelancer).transfer(salary_yocto_near)
          },
        }
      },
    }
  }
}
