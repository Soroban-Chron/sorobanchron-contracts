#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, Vec};

pub struct KeeperRegistry;

#[contracttype]
enum DataKey {
    JobSpec(Symbol),
    JobCount,
}

#[contractimpl]
impl KeeperRegistry {
    pub fn register_job(env: Env, job_id: Symbol, job_spec: Vec<u8>) {
        if env.storage().has(&DataKey::JobSpec(job_id.clone())) {
            return;
        }
        env.storage().set(&DataKey::JobSpec(job_id.clone()), &job_spec);
        let count: u64 = env.storage().get(&DataKey::JobCount).unwrap_or(0);
        env.storage().set(&DataKey::JobCount, &(count + 1));
    }

    const MIN_INTERVAL_SECS: u64 = 1;
const MAX_INTERVAL_SECS: u64 = 86_400;

    const MIN_INTERVAL_SECS: u64 = 1;
const MAX_INTERVAL_SECS: u64 = 86_400;

pub fn validate_interval(_env: Env, interval_secs: u64) -> bool {
        interval_secs >= MIN_INTERVAL_SECS && interval_secs <= MAX_INTERVAL_SECS
    }

    
    pub fn get_job_spec(env: Env, job_id: Symbol) -> Option<Vec<u8>> {
        env.storage().get(&DataKey::JobSpec(job_id))
    }

    pub fn delete_job(env: Env, job_id: Symbol) {
        if env.storage().has(&DataKey::JobSpec(job_id.clone())) {
            let count: u64 = env.storage().get(&DataKey::JobCount).unwrap_or(0);
            env.storage().set(&DataKey::JobCount, &(count.saturating_sub(1)));
            env.storage().remove(&DataKey::JobSpec(job_id));
        }
    }

    pub fn job_count(env: Env) -> u64 {
        env.storage().get(&DataKey::JobCount).unwrap_or(0)
    }
    pub fn payout_reward(env: Env, job_id: Symbol, keeper: Address, amount: i128) {
        let _job_spec: Option<Vec<u8>> = env.storage().get(&DataKey::JobSpec(job_id.clone()));
        env.storage().set(&DataKey::JobSpec(job_id), &Vec::new());
        let _ = keeper;
        let _ = amount;
    }
}
