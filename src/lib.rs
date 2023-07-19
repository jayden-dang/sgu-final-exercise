pub mod user;
pub mod client;
pub mod job;

use user::User;
use client::Client; 
use job::Job;

use near_sdk:: {
    collections::UnorderedMap,
    borsh::{self, BorshDeserialize, BorshSerialize},
    // serde::{Serialize, Deserialize},
    env, near_bindgen, AccountId, PanicOnDefault,
};

use std::vec::Vec;

// Define the contract structure
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct Contract {
    pub owner_id: AccountId,
    pub all_jobs: UnorderedMap<u64, Job>,
    pub all_jobs_per_user: UnorderedMap<AccountId, Vec<Job>>,
    pub all_jobs_per_client: UnorderedMap<AccountId, Vec<Job>>,
    pub all_users: UnorderedMap<u64, User>,
    pub all_clients: UnorderedMap<u64, Client>,
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self { 
            owner_id: env::signer_account_id(),
            all_jobs: UnorderedMap::new(b"all_jobs".to_vec()),
            all_jobs_per_user: UnorderedMap::new(b"all_jobs_per_user".to_vec()),
            all_jobs_per_client: UnorderedMap::new(b"all_jobs_per_client".to_vec()),
            all_users: UnorderedMap::new(b"all_users".to_vec()),
            all_clients: UnorderedMap::new(b"all_clients".to_vec()),
        }
    }

    pub fn view_all_users(&self) -> Vec<User>{
        let mut vec_users = Vec::new();
        for key in self.all_users.keys() {
            vec_users.push(self.all_users.get(&key).unwrap());
        }

        vec_users
    }

    pub fn view_all_clients(&self) -> Vec<Client> {
        let mut vec_clients = Vec::new();
        for key in self.all_clients.keys() {
            vec_clients.push(self.all_clients.get(&key).unwrap());
        }
        vec_clients
    } 

    pub fn view_all_jobs(&self) -> Vec<Job>{
        let mut vec_alls_job = Vec::new();
        for i in self.all_jobs.keys() {
            vec_alls_job.push(self.all_jobs.get(&i).unwrap());
        }
        vec_alls_job
    }

    pub fn remove_job_by_id(&mut self,user_id:AccountId, job_id: String) {
        let mut key = 1;
        // tìm kiếm job trong all_jobs bằng job_id
        for i in self.all_jobs.keys() {
            let job = self.all_jobs.get(&i).unwrap();
            // tìm thấy job
            if job_id == job.get_job_id() {
                // tìm client_id của job đó để để trạng thái
                let client_id = job.get_job_client_id();
                let mut vec_jobs = self.all_jobs_per_client.get(&client_id).unwrap();

                // tìm job bằng job_id trong all_jobs của client
                for idx in 1..=vec_jobs.len() {
                    if let Some(client_job) = vec_jobs.get(idx) {
                        if job_id == client_job.get_job_id() {
                            // xoá job tại vị trí và tạo job mới thêm vào
                            let job = vec_jobs.get(idx).unwrap();
                            let new_job = Job{
                                client_id: job.get_job_client_id(),
                                user_id: Some(user_id),
                                job_id: job.get_job_id(),
                                job_name: job.get_job_name(),
                                job_status: "Dang tien hanh".to_string(),
                                job_balance: job.get_job_balance(),
                            };
                            vec_jobs.remove(idx);
                            vec_jobs.push(new_job);
                            break
                        }
                    }
                }
                // ghi đè lại vector jobs của client
                self.all_jobs_per_client.insert(&job.get_job_client_id(), &vec_jobs);

                // lưu lại vị trí của job trong all_jobs
                key = i;
                break;
            }
        }
        // xoá jobs trong all_jobs
        self.all_jobs.remove(&key);
    }
}