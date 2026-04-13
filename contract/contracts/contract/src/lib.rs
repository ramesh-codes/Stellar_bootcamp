#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct CommunityFund;

#[contractimpl]
impl CommunityFund {
    // Initialize contract with an owner
    pub fn init(env: Env, owner: Address) {
        let key = symbol_short!("OWNER");
        env.storage().instance().set(&key, &owner);
    }

    // Contribute funds to the contract
    pub fn contribute(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance_key = symbol_short!("BALANCE");
        let current: i128 = env
            .storage()
            .instance()
            .get(&balance_key)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&balance_key, &(current + amount));
    }

    // Get total balance
    pub fn get_balance(env: Env) -> i128 {
        let balance_key = symbol_short!("BALANCE");
        env.storage()
            .instance()
            .get(&balance_key)
            .unwrap_or(0)
    }

    // Owner withdraws funds
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        let owner_key = symbol_short!("OWNER");
        let owner: Address = env.storage().instance().get(&owner_key).unwrap();

        owner.require_auth();

        let balance_key = symbol_short!("BALANCE");
        let current: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);

        if amount > current {
            panic!("Insufficient funds");
        }

        env.storage()
            .instance()
            .set(&balance_key, &(current - amount));

        // In real contract, you'd transfer token here
    }
}