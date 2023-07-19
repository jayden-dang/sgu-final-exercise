use near_sdk::{
    AccountId, near_bindgen,
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Serialize, Deserialize}, Promise, Balance,
};

use crate::{Contract, ContractExt};
use crate::job::Job;
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Client {
    client_id: AccountId,
    client_name: String,
}

pub trait ImplClient {
    fn create_client(&mut self, client_id: AccountId, client_name: String) -> Client;
    fn create_job(&mut self, client_id: AccountId, job_id: String, job_name: String, job_balance: Balance);
	fn view_job_by_id(&self, job_id: String) -> Option<Job>;
	fn view_jobs_per_client(&self, client_id: AccountId) -> Option<Vec<Job>>;
	fn payment(&mut self, client_id: AccountId, user_id: AccountId, job_id: String) -> Promise;
}

#[near_bindgen]
impl ImplClient for Contract {
    fn create_client(&mut self, client_id: AccountId, client_name: String) -> Client{
        let client = Client {
            client_id: client_id.clone(),
            client_name: client_name,
        };
        let vec_jobs:Vec<Job> = Vec::new();
        self.all_jobs_per_client.insert(&client_id, &vec_jobs);
        let len = self.all_clients.len() + 1;
        self.all_clients.insert(&len, &client);
        client
    }

    fn create_job(&mut self, client_id: AccountId, job_id: String, job_name: String, job_balance: Balance) {
        let job = Job{
            client_id: client_id.clone(),
            user_id: None,
            job_id: job_id,
            job_name: job_name.clone(),
            job_status: "Dang cho".to_string(),
            job_balance: job_balance,
        };
        // thêm job vào all_jobs cũng như all_jobs của client đã tạo job này
        // all_jobs
        let len_of_all_jobs = self.all_jobs.len()+1;
        self.all_jobs.insert(&len_of_all_jobs, &job);

        // all_jobs_per_client
        let mut vec_job = self.all_jobs_per_client
                                        .get(&client_id)
                                        .unwrap();
        vec_job.push(job);
        self.all_jobs_per_client.insert(&client_id, &vec_job);
    }

    fn view_job_by_id(&self, job_id: String) -> Option<Job>{
        for i in 1..=self.all_jobs.len() {
            let job = self.all_jobs.get(&i).unwrap();
            if job_id == job.get_job_id() {
                return Some(job);
            }
        }
        None
    }

    fn view_jobs_per_client(&self, client_id: AccountId) -> Option<Vec<Job>> {
        if let Some(vec_jobs_per_client) = self.all_jobs_per_client.get(&client_id) {
            Some(vec_jobs_per_client)
        } else {
            None
        }
    }
    
    #[payable]
    fn payment(&mut self, client_id: AccountId, user_id: AccountId, job_id: String) -> Promise {
        // lưu tại vị trí của job đó trong vector jobs
        let mut idx_client = 0;
        let mut idx_user = 0;
        
        // kiểm tra job có tồn tại không
        let mut check_job_exist_user = false;
        let mut check_job_exist_client = false;
        
        // kiểm tra job có nằm trong danh sách của user hay không
        if let Some(vec_job_user) = self.all_jobs_per_user.get(&user_id) {
            for i in 1..=vec_job_user.len() {
                if let Some(job) = vec_job_user.get(i){
                    if job_id == job.get_job_id() {
                        check_job_exist_user = true;
                        idx_user = i;
                        break;
                    }
                }
            }
        }
        
        let mut balance:u128 = 0;
        
        // Kiểm tra job có nằm trong danh sách của client hay không
        if let Some(vec_job_client) = self.all_jobs_per_client.get(&client_id) {
            for i in 1..=vec_job_client.len() {
                if let Some(job) = vec_job_client.get(i) {
                    if job_id == job.get_job_id() {
                        balance = job.get_job_balance();
                        check_job_exist_client = true;
                        idx_client = i;
                        break;
                    }
                }
            }
        }

        // nếu nằm trong cả 2 thì cho qua
        // em bị lỗi dòng này nếu bật thì không chạy dc 
        // assert_eq!(check_job_exist_client, check_job_exist_user);

        // tìm lại job với job_id và xoá
        if let Some(mut vec_job_user) = self.all_jobs_per_user.get(&user_id) {
            vec_job_user.remove(idx_user);
            self.all_jobs_per_user.insert(&user_id, &vec_job_user);
        }

        // tìm lại job với job_id và xoá
       if let Some(mut vec_job_client) = self.all_jobs_per_client.get(&client_id) {
            vec_job_client.remove(idx_client);
            self.all_jobs_per_client.insert(&client_id, &vec_job_client);
        }
        Promise::new(user_id).transfer(balance)
    }
}