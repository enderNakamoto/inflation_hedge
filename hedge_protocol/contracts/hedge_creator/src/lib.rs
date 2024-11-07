#![no_std]

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BytesN, Env, Symbol};

mod vault {
    soroban_sdk::contractimport!(file = "./vault.wasm");
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    HedgeInfo(BytesN<32>), // hedge_id -> HedgeInfo
}

#[derive(Clone)]
#[contracttype]
pub struct HedgeInfo {
    hedge_vault: Identifier,
    risk_vault: Identifier,
    controller: Identifier,
    maturity_date: u64,
    token_id: BytesN<32>,
}

fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(key)
}

fn read_administrator(e: &Env) -> Identifier {
    let key = DataKey::Admin;
    e.storage().get_unchecked(key).unwrap()
}

fn write_administrator(e: &Env, id: Identifier) {
    let key = DataKey::Admin;
    e.storage().set(key, id);
}

pub trait HedgeCreatorTrait {
    fn initialize(e: Env, admin: Identifier);
    
    fn create_hedge_pair(
        e: Env,
        hedge_id: BytesN<32>,
        token_id: BytesN<32>,
        maturity_date: u64,
    ) -> HedgeInfo;

    fn get_hedge_info(e: Env, hedge_id: BytesN<32>) -> HedgeInfo;
}

pub struct HedgeCreatorContract;

#[contractimpl]
impl HedgeCreatorTrait for HedgeCreatorContract {
    fn initialize(e: Env, admin: Identifier) {
        if has_administrator(&e) {
            panic!("already initialized");
        }
        write_administrator(&e, admin);
    }

    fn create_hedge_pair(
        e: Env,
        hedge_id: BytesN<32>,
        token_id: BytesN<32>,
        maturity_date: u64,
    ) -> HedgeInfo {
        // Only admin can create new hedge pairs
        let admin = read_administrator(&e);
        if admin != e.invoker().unwrap() {
            panic!("only admin can create hedge pairs");
        }

        // Validate maturity date is in the future
        if maturity_date <= e.ledger().timestamp() {
            panic!("maturity date must be in the future");
        }

        // Deploy hedge vault
        let hedge_vault_salt = e.crypto().sha256(&(hedge_id.clone(), Symbol::new(&e, "hedge")));
        let hedge_vault = e.deploy_contract(&hedge_vault_salt, vault::WASM);
        
        // Deploy risk vault
        let risk_vault_salt = e.crypto().sha256(&(hedge_id.clone(), Symbol::new(&e, "risk")));
        let risk_vault = e.deploy_contract(&risk_vault_salt, vault::WASM);
        
        // Deploy controller
        let controller_salt = e.crypto().sha256(&(hedge_id.clone(), Symbol::new(&e, "ctrl")));
        let controller = e.deploy_contract(&controller_salt, vault::WASM);

        // Initialize both vaults and controller
        vault::Client::new(&e, &hedge_vault).initialize(&admin, &token_id);
        vault::Client::new(&e, &risk_vault).initialize(&admin, &token_id);
        
        let hedge_info = HedgeInfo {
            hedge_vault: Identifier::Contract(hedge_vault),
            risk_vault: Identifier::Contract(risk_vault),
            controller: Identifier::Contract(controller),
            maturity_date,
            token_id,
        };

        // Store hedge info
        e.storage().set(&DataKey::HedgeInfo(hedge_id), &hedge_info);

        hedge_info
    }

    fn get_hedge_info(e: Env, hedge_id: BytesN<32>) -> HedgeInfo {
        e.storage()
            .get(&DataKey::HedgeInfo(hedge_id))
            .unwrap_or_else(|| panic!("hedge pair not found"))
            .unwrap()
    }
}