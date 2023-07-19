use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
    AccountId, Balance
};
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    pub client_id: AccountId,
    pub user_id: Option<AccountId>,
    pub job_id: String,
    pub job_name: String,
    pub job_status: String,
    pub job_balance: Balance,
}

impl Job {
    pub fn get_job_id(&self) -> String {
        self.job_id.clone()
    }

    pub fn get_job_client_id(&self) -> AccountId {
        self.client_id.clone()
    }

    pub fn get_job_user_id(&self) -> Option<AccountId> {
        self.user_id.clone()
    }
    
    pub fn get_job_name(&self) -> String {
        self.job_name.clone()
    }

    pub fn get_job_balance(&self) -> Balance {
        self.job_balance
    }

    pub fn set_job_user_id(&mut self, user_id: AccountId) {
        self.user_id = Some(user_id)
    }

    pub fn set_job_status(&mut self, status: String) {
        self.job_status = status
    }

}
