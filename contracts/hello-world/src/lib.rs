#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Env,
};

#[contracttype]
pub enum DataKey {
    YesVotes,
    NoVotes,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    pub fn vote_yes(env: Env) {
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::YesVotes)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::YesVotes, &(count + 1));
    }

    pub fn vote_no(env: Env) {
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NoVotes)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::NoVotes, &(count + 1));
    }

    pub fn get_results(env: Env) -> (u32, u32) {

        let yes: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::YesVotes)
            .unwrap_or(0);

        let no: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NoVotes)
            .unwrap_or(0);

        (yes, no)
    }

    pub fn reset(env: Env) {
        env.storage()
            .persistent()
            .set(&DataKey::YesVotes, &0u32);

        env.storage()
            .persistent()
            .set(&DataKey::NoVotes, &0u32);
    }
}