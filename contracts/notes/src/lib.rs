#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllocationTarget {
    pub recipient: Address,
    pub basis_points: u32, // Percentage representation where 10,000 equals 100%
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    WorkerProfile(Address), // Maps a specific worker's address to their split allocations
}

#[contract]
pub struct RemitSplitContract;

#[contractimpl]
impl RemitSplitContract {
    /// Configures or updates the remittance split rules for a specific worker
    pub fn set_profile(env: Env, worker: Address, targets: Vec<AllocationTarget>) {
        worker.require_auth();
        
        let mut total_bps: u32 = 0;
        for i in 0..targets.len() {
            let target = targets.get(i).unwrap();
            total_bps += target.basis_points;
        }

        // Must explicitly add up to exactly 100% (10,000 Basis Points)
        if total_bps != 10000 {
            panic!("Total basis points must equal exactly 10000 (100%)");
        }

        let key = DataKey::WorkerProfile(worker);
        env.storage().persistent().set(&key, &targets);
    }

    /// Executed by employers to deposit salaries and execute splits instantly on-chain
    pub fn execute_payroll(env: Env, employer: Address, worker: Address, token: Address, total_salary: i128) {
        employer.require_auth();

        if total_salary <= 0 {
            panic!("Salary must be greater than zero");
        }

        let key = DataKey::WorkerProfile(worker.clone());
        if !env.storage().persistent().has(&key) {
            panic!("Worker has not configured their remittance profile");
        }

        let targets: Vec<AllocationTarget> = env.storage().persistent().get(&key).unwrap();
        let client = token::Client::new(&env, &token);

        // First pull the complete salary amount from the employer into this contract account
        client.transfer(&employer, &env.current_contract_address(), &total_salary);

        let mut distributed_amount: i128 = 0;

        // Loop through each dependent destination and route their specified share
        for i in 0..targets.len() {
            let target = targets.get(i).unwrap();
            
            // Calculate slice using safe math: (salary * bps) / 10000
            let share = (total_salary * target.basis_points as i128) / 10000;
            
            if share > 0 {
                client.transfer(&env.current_contract_address(), &target.recipient, &share);
                distributed_amount += share;
            }
        }

        // If math rounding leaves any micro-dust behind, push