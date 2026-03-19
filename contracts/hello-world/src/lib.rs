#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Stake(Address),
    TotalStaked,
}

#[contract]
pub struct TokenStakingPlatformContract;

#[contractimpl]
impl TokenStakingPlatformContract {
    pub fn stake(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let mut user_stake: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Stake(user.clone()))
            .unwrap_or(0);

        user_stake += amount;

        env.storage()
            .instance()
            .set(&DataKey::Stake(user.clone()), &user_stake);

        let total_staked: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalStaked)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TotalStaked, &(total_staked + amount));
    }

    pub fn withdraw(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let mut user_stake: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Stake(user.clone()))
            .unwrap_or(0);

        if user_stake < amount {
            panic!("Insufficient staked balance");
        }

        user_stake -= amount;

        env.storage()
            .instance()
            .set(&DataKey::Stake(user.clone()), &user_stake);

        let total_staked: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalStaked)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TotalStaked, &(total_staked - amount));
    }

    pub fn get_stake(env: Env, user: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Stake(user))
            .unwrap_or(0)
    }

    pub fn get_total_staked(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalStaked)
            .unwrap_or(0)
    }
}