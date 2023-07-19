use crate::{Contract, ContractExt};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId,
    near_bindgen, 
};
use crate::job::Job;
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub user_id: AccountId,
    pub user_name: String,
}

pub trait ImplUser {
    fn create_user(&mut self, user_id: AccountId, user_name: String) -> User;
    fn take_job(&mut self, user_id: AccountId ,job_id: String) -> String;
    fn view_jobs_per_user(&self, user_id: AccountId) -> Option<Vec<Job>>;
    fn complete_job(&mut self, user_id: AccountId, job_id: String);
}

#[near_bindgen]
impl ImplUser for Contract {
    fn create_user(&mut self, user_id: AccountId, user_name: String) -> User{
        let user = User { 
            user_id: user_id.clone(),
            user_name: user_name,
        };

        let vec_jobs:Vec<Job> = Vec::new();
        self.all_jobs_per_user.insert(&user_id, &vec_jobs);
        let len = self.all_users.len() + 1;
        self.all_users.insert(&len, &user);
        user
    }

    fn take_job(&mut self, user_id: AccountId, job_id: String) -> String {
        // flag check để tìm job tồn tại, sau đó xoá job trong all_job
        // chuyển trạng thái của job trong all_jobs của client
        let mut check = false;
        // tìm kiếm job trong tất cả các job
        for i in self.all_jobs.keys() {
            let mut job = self.all_jobs.get(&i).unwrap();
            if job_id == job.get_job_id() {
                // chuyển trạng thái của job được tìm thấy bằng job_id
                job.set_job_user_id(user_id.clone());
                job.set_job_status("Dang tien hanh".to_string());
                // lưu và vector job của user
                let mut vec_jobs = self.all_jobs_per_user.get(&user_id).unwrap();
                vec_jobs.push(job);

                // ghi vào all_jobs của user
                self.all_jobs_per_user.insert(&user_id, &vec_jobs);

                // chec
                check = true;
            }
        }
        if check {
            // hàmg xoá job trong all_jobs đồng thời chuyển trạng thái của job trong all_job client
            self.remove_job_by_id(user_id, job_id);
            return "Nhan job thanh cong".to_string();
        } else {
            "Job khong ton tai".to_string()
        }
    }

    fn view_jobs_per_user(&self, user_id: AccountId) -> Option<Vec<Job>> {
        if let Some(vec_jobs) = self.all_jobs_per_user.get(&user_id) {
            Some(vec_jobs)
        } else {
            None
        }
    }

    // thay đổi trạng thái đã hoàn thành job
    fn complete_job(&mut self, user_id: AccountId, job_id: String) {
        let mut idx_user = 0;

        // tạo 1 client để lưu giá trị tạm
        let mut client_id = user_id.clone();

        // thuộc USER
        // tìm job trong all_jobs của user
        if let Some(mut vec_jobs_per_user) = self.all_jobs_per_user.get(&user_id) {
            for i in 1..=vec_jobs_per_user.len() {
                if let Some(job) = vec_jobs_per_user.get(i) {
                    if job_id == job.get_job_id() {
                        // tìm thấy job và lưu lại vị trí
                        idx_user = i;
                        break;
                    }
                } 
            }
            let job_temp = vec_jobs_per_user.get(idx_user).unwrap();
            client_id = job_temp.get_job_client_id();
            // tạo job mới
            let new_job = Job {
                client_id: job_temp.get_job_client_id(),
                user_id: Some(user_id.clone()),
                job_id: job_temp.get_job_id(),
                job_name: job_temp.get_job_name(),
                job_status: "Hoan thanh".to_string(),
                job_balance: job_temp.get_job_balance(),
            };

            // xoá job tại vị trí và thêm và job mới
            vec_jobs_per_user.remove(idx_user);
            vec_jobs_per_user.push(new_job);
            // ghi đè lại vector job của user_id
            self.all_jobs_per_user.insert(&user_id, &vec_jobs_per_user);
        }
        
        // thuộc CLIENT
        let mut idx_client = 0;
        // tìm kiếm job trong all_job của client
        if let Some(mut vec_jobs_per_client) = self.all_jobs_per_client.get(&client_id) {
            for i in 1..=vec_jobs_per_client.len() {
                if let Some(job) = vec_jobs_per_client.get(i){
                    if job_id == job.get_job_id() {
                        // tìm thấy job và lưu lại vị trí
                        idx_client = i;
                        break;
                    }
                }
            }

            let job_temp = vec_jobs_per_client.get(idx_client).unwrap();
            // tạo job mới với 1 phần giá trị của job_temp được lấy ra
            let new_job = Job {
                client_id: job_temp.get_job_client_id(),
                user_id: Some(user_id.clone()),
                job_id: job_temp.get_job_id(),
                job_name: job_temp.get_job_name(),
                job_status: "Hoan thanh".to_string(),
                job_balance: job_temp.get_job_balance(),
            };
            // xoá job tại vị trí đã lưu và thêm vào job mới
            vec_jobs_per_client.remove(idx_client);
            vec_jobs_per_client.push(new_job);
            //ghi đề vector job của all_jobs client
            self.all_jobs_per_client.insert(&client_id, &vec_jobs_per_client);
        }
    }
}