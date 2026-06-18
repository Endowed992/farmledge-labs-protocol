#![no_std]

mod errors;
mod storage;

pub use errors::ContractError;
use storage::DataKey;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct MaizeReceiptContract;

#[contractimpl]
impl MaizeReceiptContract {
    pub fn init(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenCounter, &0u64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_init_sets_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MaizeReceiptContract);
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let stored: Address = env
            .as_contract(&contract_id, || {
                env.storage().instance().get(&DataKey::Admin).unwrap()
            });
        assert_eq!(stored, admin);
    }

    #[test]
    fn test_init_sets_counter_to_zero() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MaizeReceiptContract);
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let counter: u64 = env
            .as_contract(&contract_id, || {
                env.storage().instance().get(&DataKey::TokenCounter).unwrap()
            });
        assert_eq!(counter, 0u64);
    }

    #[test]
    fn test_init_rejects_double_call() {
        let env = Env::default();
        let contract_id = env.register_contract(None, MaizeReceiptContract);
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let result = client.try_init(&admin);
        assert_eq!(
            result,
            Err(Ok(ContractError::AlreadyInitialized))
        );
    }

    #[test]
    fn test_datakey_variants_exist() {
        let env = Env::default();
        let _admin = DataKey::Admin.clone();
        let _custodians = DataKey::Custodians.clone();
        let _token_meta = DataKey::TokenMeta(String::from_str(&env, "token_id"));
        let _owner = DataKey::Owner(String::from_str(&env, "token_id"));
        let _token_counter = DataKey::TokenCounter.clone();

        let _ = _admin;
        let _ = _custodians;
        let _ = _token_meta;
        let _ = _owner;
        let _ = _token_counter;
    }
}
